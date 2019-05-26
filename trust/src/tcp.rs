use std::io;
use std::io::prelude::*;


pub enum State {
	// Closed,
	// Listen,
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
	ip: etherparse::Ipv4Header,
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
		nic: &mut tun_tap::Iface,
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
			// after get the SYN request from client, 
			// the server state gonna be  SynRcvd ( checkout the RFC)
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
			ip: etherparse::Ipv4Header::new(
				0, 
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
			),
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
		c.ip.set_payload_len(syn_ack.header_len() as usize + 0/* data len*/);
		

		// kernel does this already
		// syn_ack.checksum = syn_ack.calc_checksum_ipv4(&ip, &[])
		// 						  .expect("failed to compute checksum");

		// write out the headers
		let unwritten = {
			let mut unwritten = &mut buf[..];
			c.ip.write(& mut unwritten);
			syn_ack.write(& mut unwritten);
			unwritten.len()
		};

		// send ipv4header to client
		/* modern os defend SYN-flood by not allocating any local resour until 
			the connection is established
		*/
		nic.send(&buf[..unwritten])?;

		Ok(Some(c))
	}

	pub fn on_packet<'a> (
		&mut self,
		nic: &mut tun_tap::Iface,
		iph: etherparse::Ipv4HeaderSlice<'a>, 
		tcph: etherparse::TcpHeaderSlice<'a>, 
		data: &'a [u8]
	) -> io::Result< () > {
		/* 
			fist, check that sequence numbers are valid (RFC S3.3)
			acceptable ack check
		   	SND.UNA < SEG.ACK =< SND.NXT
		   	but remember wrapping
		*/ 
		let ackn = tcph.acknowledgment_number();
		if self.send.una < ackn {

			// check is violated iff n is between u and a	
			// hard to understand ...
			// it's better to draw it out
			if self.send.nxt >= self.send.una && self.send.nxt < ackn {
				return Ok(());
			}
		
		} else {
			// wrap around because una >= ackn
			// check is Okay iff n is betwween u and a
			if self.send.nxt >= ackn && self.send.nxt < self.send.una {

			} else {
				return Ok(());
			}
		}
		// it's should not be so easy ...
		// if !(self.send.una < tcph.acknowledgment_number()  && tcph.acknowledgment_number() <= self.send.nxt) {
		// 	return Ok(())
		// }



		match self.state {
			State::SynRcvd => {
				// expect to get an ACK from our SYN
				
			}
			State::Estab => {
				unimplemented!();
			}
		}

		Ok(())
	}

}
