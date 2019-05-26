use std::io;
use std::collections::HashMap;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
	let mut connections: HashMap<Quad, tcp::Connection> = Default::default();
	let mut nic = tun_tap_mac::Iface::without_packet_info("tun0", tun_tap_mac::Mode::Tun)?;
	// let nic = tun_tap_mac::Iface::new("tun0", tun_tap_mac::Mode::Tun)?;
	let mut buf = [0u8; 1504];
	loop{
	    let nbytes = nic.recv(&mut buf[..])?;
		//let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
		//let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
		//if eth_proto != 0x0800 {
		//	// no ipv4, link level protocol
		//	continue;
		//}

		// 解析ip header
		//match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]){
			match etherparse::Ipv4HeaderSlice::from_slice(&buf[..nbytes]){
			 Ok(iph) => {
				 let src = iph.source_addr();
				 let dst = iph.destination_addr();
				 if iph.protocol() != 0x06 {
					// not tcp
					continue;
				 }
				
				// 解析tcp header
				let ip_hdr_sz = iph.slice().len();
				 match etherparse::TcpHeaderSlice::from_slice(&buf[iph.slice().len()..nbytes]) {
					Ok(tcph) => {
						use std::collections::hash_map::Entry;
						let datai = iph.slice().len() + tcph.slice().len();
						match connections.entry( Quad {
								src: (src, tcph.source_port()),
								dst: (dst, tcph.destination_port()),
							}) {
								Entry::Occupied(mut c) => {
									c.get_mut().on_packet(&mut nic, iph, tcph, &buf[datai..nbytes])?;		
								},
								Entry::Vacant(mut e) => {
									if let Some(c) = tcp::Connection::accept(&mut nic, iph, tcph, &buf[datai..nbytes]) ? {
									e.insert(c);
									}
								},
							}
							
							
				        // (srcip, srcport, dstip, dstport)
					},
					Err(e) => {
				        eprintln!("ignore weird tcp packet {:?}", e)
					}

				 }
			 },
		     Err(e) => {
				 eprintln!("ignore weird packet {:?}", e)
			 }
		}
		
	}
	//Ok(())
}
