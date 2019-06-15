use std::io;
use std::io::prelude::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::collections::{VecDeque, HashMap};

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

struct InterfaceHandle = Arc<Mutex<ConnectionManager>>;

pub struct Interface{
    ih: InterfaceHandle,    // lock todo read & writes
    jh: thread::JoinHandle<()>,
}

#[#[derive(Default)]]
struct ConnectionManager {
    connections: HashMap<Quad, tcp::Connection>,
    pendding: HashMap<u16, Vec<Quad>>,
}

impl Interface {
    pub fn new() -> io::Result<Self> {
        let nic = tun_tap::Iface::without_packet_info("tun0", tun_tap::Mode::Tun)?,
        
        let cm: InterfaceHandle = Arc::default();

        let jh = {
            let cm = cm.clone()
            thread::spawn(move || {
                let nic = nic;
                let buf = [u8, 1504],

                // do the stuff that main does
            });
        };

        Ok(Interface { cm, jh})
    }

    pub fn bind(&mut self, port: u16) -> io::Result<TcpListener> {
        use std::collections::hash_map::Entry;
        let cm = self.ih.lock().unwrap();
        match cm.pendding.insert(port) {
            Entry::Vacant(v) => {
                v.insert(VecDeque::new());
            },
            Entry::Occupied(_) => {
                return Err(io::Error::new(io::ErrorKind::AddrInUse, "port already bounrd"));
            };
        };
        drop(cm);
        Ok(TcpListener(port, self.ih.clone()));
    }
}

struct TcpListener(u16, InterfaceHandle);

impl TcpListener {
    pub fn try_accept(&mut self) -> io::Result<TcpStream> {
        let cm = self.ih.lock().unwrap();
        if let Some(quad) = cm.pendding.get_mut(&self.0).expect("port closed while listenner still active").pop_front() {
            return Ok(TcpStream(quad, self.1.clone()));
        } else {
            // TODO: Lock
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "no conneciton to accept"
            ));
            
        }
    }
}

struct TcpStream(Quad, InterfaceHandle);

impl Read for TcpStream{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let cm = self.ih.lock().unwrap();
        let c = cm.connections.get_mut(&self.0).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted, 
                "stream was terminated unexpectedly!"
            )
        })?;

        if c.incomming.is_empty() {
            // TODO: block
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "no bytes to read",
            ));
        }

        

        let (read, rx) = mpsc::channel();
        self.1.send(InterfaceRequest::Read {
            quad: self.quad,
            max_length: buf.len(),
            read, 
        });
        let bytes = rx.recv().unwrap();
        assert!(bytes.len() <= buf.len());
        buf.copy_from_slice(&bytes[...]);
        Ok(bytes.len())
    }
}

impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let (ack, rx) = mpsc::channel();
        self.1.send(InterfaceRequest::Write {
            quad: self.quad,
            bytes: Vec::from(buf),
            ack, 
        });
        let n = rx.recv().unwrap();
        assert!(n <= buf.len());
        Ok(n)
    }
    
    fn flush(&mut self) -> io::Result<()> {
        let (ack, rx) = mpsc::channel();
        self.1.send(InterfaceRequest::Flush {
            quad: self.quad,
            ack, 
        });
        rx.recv().unwrap();
        Ok(())
    }
}
