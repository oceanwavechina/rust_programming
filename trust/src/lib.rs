use std::collections::{VecDeque, HashMap};
use std::io;
use std::io::prelude::*;
use std::sync::{Mutex, Arc, Condvar};
use std::thread;
use std::net::Ipv4Addr;

mod tcp;


const SENDQUEUE_SIZE: usize = 1024;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

#[derive(Default)]
struct Foobar {
    manager: Mutex<ConnectionManager>,
    pending_var: Condvar,
    rcv_var: Condvar,
}

type InterfaceHandle = Arc<Foobar>;

pub struct Interface{
    ih: Option<InterfaceHandle>,    // lock todo read & writes
    jh: Option<thread::JoinHandle<io::Result<()>>>,
}

impl Drop for Interface {
    fn drop(&mut self) {
        self.ih.as_mut().unwrap().manager.lock().unwrap().terminate = true;
        
        drop(self.ih.take());
        self.jh
            .take()
            .expect("interface dropped more than once")
            .join()
            .unwrap()
            .unwrap();
    }
}


#[derive(Default)]
struct ConnectionManager {
    terminate: bool,
    connections: HashMap<Quad, tcp::Connection>,
    pendding: HashMap<u16, VecDeque<Quad>>,
}

fn packet_loop(mut nic: tun_tap::Iface, ih: InterfaceHandle) -> io::Result<()> {
    let mut buf = [0u8; 1504];
    
    loop{
        // TODO timeout for recv
	    let nbytes = nic.recv(&mut buf[..])?;
		
		// if s/without_packet_info/new/:

		// let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
		// let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
		// if eth_proto != 0x0800 {
		//	// no ipv4, link level protocol
		//	continue;
		// }
		// and also include on send

		// 解析ip header
		match etherparse::Ipv4HeaderSlice::from_slice(&buf[..nbytes]){
			Ok(iph) => {
				let src = iph.source_addr();
				let dst = iph.destination_addr();
				if iph.protocol() != 0x06 {
					println!("BAD PRPTOCOL");
				// not tcp
				continue;
				}
			
			// 解析tcp header
			let ip_hdr_sz = iph.slice().len();
				match etherparse::TcpHeaderSlice::from_slice(&buf[iph.slice().len()..nbytes]) {
				Ok(tcph) => {
					use std::collections::hash_map::Entry;
					let datai = iph.slice().len() + tcph.slice().len();
                    let mut cmg = ih.manager.lock().unwrap();
                    let mut cm = &mut *cmg;
                    let q = Quad {
							src: (src, tcph.source_port()),
							dst: (dst, tcph.destination_port()),
						};
					match cm.connections.entry(q) {
							Entry::Occupied(mut c) => {
                                let a = c.get_mut().on_packet(
                                    &mut nic,
                                    iph,
                                    tcph,
                                    &buf[datai..nbytes]
                                )?;
                                drop(cmg);
                                if a.contains(tcp::Available::READ) {
                                    ih.rcv_var.notify_all();
                                }
                                if a.contains(tcp::Available::WRITE) {
                                    //ih.snd _var.notify_all();
                                }
							},
							Entry::Vacant(e) => {
                                if let Some(pendding) = cm.pendding.get_mut(&tcph.destination_port()) {
                                    if let Some(c) = tcp::Connection::accept(
                                        &mut nic,
                                        iph,
                                        tcph,
                                        &buf[datai..nbytes]) ? 
                                    {
                                        e.insert(c);
                                        pendding.push_back(q);
                                        drop(cmg);
                                        ih.pending_var.notify_all();
                                    }
                                }
							}
						}
						
						
					// (srcip, srcport, dstip, dstport)
				},
				Err(e) => {
					eprintln!("ignore weird tcp packet {:?}", e)
				}

				}
			},
			Err(e) => {
				//eprintln!("ignore weird packet {:?}", e)
			}
	    }
	}
}

impl Interface {
    pub fn new() -> io::Result<Self> {
        let nic = tun_tap::Iface::without_packet_info("tun0", tun_tap::Mode::Tun)?;
        
        let ih: InterfaceHandle = Arc::default();

        let jh = {
            let ih = ih.clone();
            thread::spawn(move || {
                packet_loop(nic, ih)
            })
        };

        Ok(Interface{ih: Some(ih), jh: Some(jh)})
    }

    pub fn bind(&mut self, port: u16) -> io::Result<TcpListener> {
        use std::collections::hash_map::Entry;
        let mut cm = self.ih.as_mut().unwrap().manager.lock().unwrap();
        match cm.pendding.entry(port) {
            Entry::Vacant(v) => {
                v.insert(VecDeque::new());
            }
            Entry::Occupied(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::AddrInUse,
                    "port already bounrd"
                ));
            }
        };
        drop(cm);
        Ok(TcpListener {
            port, 
            h: self.ih.as_mut().unwrap().clone(),
        })
    }
}

pub struct TcpListener {
    port: u16,
    h: InterfaceHandle,
}

impl Drop for TcpListener {
    fn drop(&mut self) {
        let mut cm = self.h.manager.lock().unwrap();
        let pending = cm.pendding
            .remove(&self.port)
            .expect("port closed while listenner still active");
        for quad in pending {
            // tODO teminate cm.connecions[quad]
            unimplemented!();
        }
    }
}

impl TcpListener {
    pub fn accept(&mut self) -> io::Result<TcpStream> {
        let mut cm = self.h.manager.lock().unwrap();
        loop {
            if let Some(quad) = cm
                .pendding
                .get_mut(&self.port)
                .expect("port closed while listenner still active")
                .pop_front() 
            {
                return Ok(TcpStream {
                    quad,
                    h: self.h.clone(),
                });
            }
            
            cm = self.h.pending_var.wait(cm).unwrap();
        }
    }
}

pub struct TcpStream {
    quad: Quad,
    h: InterfaceHandle,
}


impl Drop for TcpStream {
    fn drop(&mut self) {
        let mut cm = self.h.manager.lock().unwrap();
        // TODO:send fin on cmd.connections[quad]
        // if let Some(c) = cm.connections.remove(&self.quad) {    
        //     //unimplemented!();
        // }
    }
}

impl Read for TcpStream{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut cm = self.h.manager.lock().unwrap();
        loop {
            let c = cm.connections.get_mut(&self.quad).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::ConnectionAborted, 
                    "stream was terminated unexpectedly!"
                )
            })?;

            if c.is_rcv_closed() && c.incoming.is_empty() {    
                // peer closed
                return Ok(0);
            }
            
            if !c.incoming.is_empty() {
                // TODO: detect fin and return nread == 0
                let mut nread = 0;
                let (head, tail) = c.incoming.as_slices();
                let hread = std::cmp::min(buf.len(), head.len());
                buf[..hread].copy_from_slice(&head[..hread]);
                nread += hread;
                let tread = std::cmp::min(buf.len()-nread, tail.len());
                buf[hread..(hread+tread)].copy_from_slice(&tail[..tread]);
                nread += tread;
                drop(c.incoming.drain(..nread));
                return Ok(nread);
            }
            
            cm = self.h.rcv_var.wait(cm).unwrap();
        }
    }
}

impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut cm = self.h.manager.lock().unwrap();
        let c = cm.connections.get_mut(&self.quad).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted, 
                "stream was terminated unexpectedly!"
            )
        })?;

        if c.unacked.len() > SENDQUEUE_SIZE{
            // TODO: block
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "too many bytes buffered",
            ));
        }

        let nwrite = std::cmp::min(buf.len(), SENDQUEUE_SIZE - c.unacked.len());
        c.unacked.extend(buf[..nwrite].iter());
        Ok(nwrite)
    }
    
    fn flush(&mut self) -> io::Result<()> {
        let mut cm = self.h.manager.lock().unwrap();
        let c = cm.connections.get_mut(&self.quad).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted, 
                "stream was terminated unexpectedly!"
            )
        })?;

        if c.unacked.is_empty() {
            Ok(())
        } else {
            // TODO: block
            Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "too many bytes buffered",
            ))
        }
    }
}

impl TcpStream {
    pub fn shutdown(&self, how: std::net::Shutdown) -> io::Result<()> {
        // TOOD: send fin
        unimplemented!();
    }
}