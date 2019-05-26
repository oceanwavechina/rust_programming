use std::io;
use std::io::prelude::*;


pub enum State {
	Closed,
	Listen,
	SynRcvd,
	Estab,
}

/* tcprfc: Transmission Control Block (p10)
	https://tools.ietf.org/html/rfc793
*/
pub struct Connection {
	state: State,
	send: SendSequeceSpace,
	recv: RecvSequenceSpace,
}

///
///	RFC 793 S3.2
///	State of Send Sequence Space
/// ```
///				1         2          3          4
///			----------|----------|----------|----------
///					SND.UNA    SND.NXT    SND.UNA
///										+SND.WND
///
///	1 - old sequence numbers which have been acknowledged
///	2 - sequence numbers of unacknowledged data
///	3 - sequence numbers allowed for new data transmission
///	4 - future sequence numbers which are not yet allowed
///
///						Send Sequence Space
///
///							Figure 4.
/// ```
struct SendSequeceSpace {
	/// send unacknowleged
	una: u32,
	/// send next
	nxt: u32,
	/// send window
	wnd: u16,
	/// send urgent pointer
	up: bool,
	/// segment sequence number used for last window update
	wl1: usize,
	/// segment acknowledgemnt number used for last window update
	wl2: usize,
	/// initial send sequence number
	iss: u32
}

/// RFC 793 S3.2
/// Receive Sequence Space
///  ```
/// 					1          2          3
/// 				----------|----------|----------
/// 						RCV.NXT    RCV.NXT
/// 								+RCV.WND
/// 
/// 	1 - old sequence numbers which have been acknowledged
/// 	2 - sequence numbers allowed for new reception
/// 	3 - future sequence numbers which are not yet allowed
/// 
/// 						Receive Sequence Space
/// 
/// 							Figure 5.
/// ```
struct RecvSequenceSpace {
	/// receive next
	nxt: u32,
	/// receive window
	wnd: u16,
	/// receive urgent pointer
	up: bool,
	/// initial receive sequence number
	irs: u32,
}

impl Connection {
	pub fn accept<'a> (
		nic: &mut tun_tap_mac::Iface,
		iph: etherparse::Ipv4HeaderSlice<'a>, 
		tcph: etherparse::TcpHeaderSlice<'a>, 
		data: &'a [u8]
	) -> io::Result< Option<Self> > {
		
		let mut buf =  [0u8; 1500];
			
		if !tcph.syn() {
			// onlyn expected SYN packet
			return Ok(None);
		}

		let iss = 0;
		let mut c = Connection{
			state: State::SynRcvd,
			send: SendSequeceSpace {
				iss,
				una: iss,
				nxt: iss + 1,
				wnd: 10,
				up: false,

				wl1: 0,
				wl2:0,
			},
			recv: RecvSequenceSpace {
				// keep track of sender info
				irs: tcph.sequence_number(),
				nxt: tcph.sequence_number() +1,
				wnd: tcph.window_size(),
				up: false,
			},
		};

		
		// need to establish a connection
		// https://docs.rs/etherparse/0.8.2/etherparse/struct.TcpHeader.html
		let mut syn_ack = etherparse::TcpHeader::new(
			tcph.destination_port(), 
			tcph.source_port(), 
			c.send.iss, // random
			c.send.wnd,
		);
		// 这里返回的ack是clientsyn的number的下一个，详见 RFC 793
		syn_ack.acknowledgment_number = c.recv.nxt;
		syn_ack.syn = true;
		syn_ack.ack = true;

		// send ipv4header to client
		/* modern os defend SYN-flood by not allocating any local resour until 
			the connection is established
		*/
		let mut ip = etherparse::Ipv4Header::new(
			syn_ack.header_len(), 
			64, 
			etherparse::IpTrafficClass::Tcp, 
			[
				iph.destination()[0],
				iph.destination()[1],
				iph.destination()[2],
				iph.destination()[3],
			],
			[
				iph.source()[0],
				iph.source()[1],
				iph.source()[2],
				iph.source()[3],
			]
		);

		// kernel does this already
		// syn_ack.checksum = syn_ack.calc_checksum_ipv4(&ip, &[])
		// 						  .expect("failed to compute checksum");

		eprintln!("got ip header:\n {:02x?}", iph);
		eprintln!("got tcp header:\n {:02x?}", tcph);

		// write out the headers
		let unwritten = {
			let mut unwritten = &mut buf[..];
			ip.write(& mut unwritten);
			syn_ack.write(& mut unwritten);
			unwritten.len()
		};

		eprintln!("responding with {:02x?}", &buf[..buf.len() -unwritten]);

		nic.send(&buf[..unwritten])?;

		Ok(Some(c))
		// eprintln!("{}:{} -> {}:{} {}b of tcp", 
		// 	iph.source_addr(), tcph.source_port(),
		// 	iph.destination_addr(), tcph.destination_port(),
		// 	data.len());
	}

	pub fn on_packet<'a> (
		&mut self,
		nic: &mut tun_tap_mac::Iface,
		iph: etherparse::Ipv4HeaderSlice<'a>, 
		tcph: etherparse::TcpHeaderSlice<'a>, 
		data: &'a [u8]
	) -> io::Result< () > {
		//unimplemented!();
		Ok(())
	}

}
