use std::io;
use std::io::prelude::*;


pub enum State {
	// Closed,
	// Listen,
	SynRcvd,
	Estab,
	FinWait1,
	Closeing,
}

impl State {
	fn is_non_synchronized(&self) -> bool {
		match *self {
			State::SynRcvd => false,
			State::Estab | State::FinWait1 | State::Closeing => true,
		}
	}
}

/* tcprfc: Transmission Control Block (p10)
	https://tools.ietf.org/html/rfc793
*/
pub struct Connection {
	state: State,
	send: SendSequeceSpace,
	recv: RecvSequenceSpace,
	ip: etherparse::Ipv4Header,
	tcp: etherparse::TcpHeader,
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
		let wnd = 10;
		let mut c = Connection{
			// after get the SYN request from client, 
			// the server state gonna be  SynRcvd ( checkout the RFC)
			state: State::SynRcvd,
			send: SendSequeceSpace {
				iss,
				una: iss,
				nxt: iss,
				wnd: wnd,
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
			tcp: etherparse::TcpHeader::new(
				tcph.destination_port(), 
				tcph.source_port(), 
				iss, // random
				wnd,
			);
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
		self.tcp.syn = true;
		self.tcp.ack = true;
		c.write(nic, &[])?;

		Ok(Some(c))
	}

	fn write(&mut self, nic:&mut tun_tap::Iface, payload: &[u8]) -> io::Result<(usize)> {
		let mut buf =  [0u8; 1500];

		self.tcp.sequence_number = self.send.nxt;
		// https://docs.rs/etherparse/0.8.2/etherparse/struct.TcpHeader.html
			// 这里返回的ack是clientsyn的number的下一个，详见 RFC 793
		self.tcp.acknowledgment_number = self.recv.nxt;

		let size = std::cmp::min(
			buf.len(), 
			self.tcp.header_len() as usize + self.ip.header_len() as usize + payload.len()
		);

		self.ip.set_payload_len(size);

		// kernel does this already
		// self.tcp.checksum = self.tcp.calc_checksum_ipv4(&self.ip, &[])
		// 						  .expect("failed to compute checksum");

		// write out the headers
		use std::io::Write;
		let mut unwritten = &mut buf[..];	// move to next writefull point we have not written yet
		self.ip.write(& mut unwritten);
		self.tcp.write(& mut unwritten);
		let payload_bytes = unwritten.write(payload)?;
		let unwritten = unwritten.len();
		self.send.nxt = self.send.nxt.wrapping_add(payload_bytes as u32);
		if self.tcp.syn() {
			self.send.nxt = self.send.nxt.wrapping_add(1);
			self.tcp.syn = false;
		}
		if self.tcp.fin() {
			self.send.nxt = self.send.nxt.wrapping_add(1);
			self.tcp.fin = false;
		}

		nic.send(&buf[..buf.len()-unwritten]);
		Ok(payload_bytes);

		// send ipv4header to client
		/* modern os defend SYN-flood by not allocating any local resour until 
			the connection is established
		*/
	}

	fn send_rst( &mut self, nic: &mut tun_tap::Iface, ) -> io::Result< () > {
		self.tcp.rst = true;

		// TODO: fix sequcence number here
		/** 
			If the incoming segment has an ACK field, the reset takes its
			sequence number from the ACK field of the segment, otherwise the
			reset has sequence number zero and the ACK field is set to the sum
			of the sequence number and segment length of the incoming segment.
			The connection remains in the CLOSED state.
		 */
		// TODO: handle synchronized RST
		/** 
		 	3.  If the connection is in a synchronized state (ESTABLISHED,
			FIN-WAIT-1, FIN-WAIT-2, CLOSE-WAIT, CLOSING, LAST-ACK, TIME-WAIT),
			any unacceptable segment (out of window sequence number or
			unacceptible acknowledgment number) must elicit only an empty
			acknowledgment segment containing the current send-sequence number
			and an acknowledgment indicating the next sequence number expected
			to be received, and the connection remains in the same state.
		 */
		self.tcp.sequence_number = 0;
		self.tcp.acknowledgment_number = 0;
		self.ip.set_payload_len((self.tcp.header_len()));
		self.write(nic, &[])?;
		Ok(())
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
		if !is_between_wrapped(self.send.una, ackn, self.send.nxt.wrapping_add(1)) {
			if !self.state.is_non_synchronized() {
				// according to Reset Generation, we should send a RST
				self.send_rst(nic);
			}
			return Ok(());
			// return Err((io::Error::new(io::ErrorKind::BrokenPipe, "tried to ack unsent byte")));
		}

		/*
			valid segment check, okay if it acks at least on byte,
			which means that at least one of the follwoing is true:
				RCV.NXT =< SEG.SEQ < RCV.NXT+RCV.WND  (the first byte of the segment)
				RCV.NXT =< SEG.SEQ+SEG.LEN-1 < RCV.NXT+RCV.WND  (the last byte of the segment)
		*/
		let seqn = tcph.sequence_number();
		let mut slen = data.len() as u32;
		if tcph.fin() {
			slen += 1;
		};
		if tcph.syn() {
			slen += 1;
		};
		let wend = self.recv.nxt.wrapping_add(self.recv.wnd as u32);
		if slen == 0 {
			// zero length segment has  sperate rules for acceptance
			if self.recv.wnd == 0 {
				if seqn != self.recv.nxt {
					return Ok(());
				}
			} else if !is_between_wrapped(self.recv.nxt.wrapping_add(1), seqn, wend) {
				return Ok(());		
			}
		} else {
			if self.recv.wnd == 0 {
				return Ok(());
			} else {
				if !is_between_wrapped(self.recv.nxt.wrapping_add(1), seqn, wend) &&
					!is_between_wrapped(self.recv.nxt.wrapping_add(1), seqn+data.len() as u32 - 1, wend) 
				{
					return Ok(());
				}		
			}
		}
		
		if !is_between_wrapped(self.recv.nxt.wrapping_add(1), seqn, wend) &&
		   !is_between_wrapped(self.recv.nxt.wrapping_add(1), seqn+slen - 1, wend) {
			return Ok(());
		}

		/** 
		    (RFC793 p29)

				TCP A                                                TCP B

			1.  CLOSED                                               LISTEN

			2.  SYN-SENT    --> <SEQ=100><CTL=SYN>               --> SYN-RECEIVED

			3.  ESTABLISHED <-- <SEQ=300><ACK=101><CTL=SYN,ACK>  <-- SYN-RECEIVED

			4.  ESTABLISHED --> <SEQ=101><ACK=301><CTL=ACK>       --> ESTABLISHED

			5.  ESTABLISHED --> <SEQ=101><ACK=301><CTL=ACK><DATA> --> ESTABLISHED

					Basic 3-Way Handshake for Connection Synchronization

											Figure 7.
		*/

		match self.state {
			State::SynRcvd => {
				// expect to get an ACK from our SYN
				if !tcph.ack() {
					return Ok(());
				}
				// must have  ACKed our SYN, since we detected at least one acked byte
				// and we have only sent one byte (the SYN)
				// the three-way handshake finished !!!
				self.state = State::Estab;

				// now let's terminate the connection!
				// TODO: needs to be stored in the retransmission queue!
				// because fin must be sent after the data, if any
				self.tcp.fin = true;
				self.write(nic, &[])?;
				self.state = State::FinWait1;
			}
			State::Estab => {
				unimplemented!();
			}
			State::FinWait1 => {
				if !tcph.fin() || data.is_empty() {
					unimplemented!();
				}

				// must have  ACKed our SYN, since we detected at least one acked byte
				// and we have only sent one byte (the SYN)
				self.tcp.fin = false;
				self.write(nic, &[])?;
				self.state = State::Closeing;
			}
		}

		Ok(())
	}

}

fn is_between_wrapped(start: u32, x: u32, end: u32) -> bool {

	// it's should not be so easy ...
	// if !(self.send.una < tcph.acknowledgment_number()  && tcph.acknowledgment_number() <= self.send.nxt) {
	// 	return Ok(())
	// }

	use std::cmp::Ordering;
	match start.cmp(x) {
		Ordering::Equal => return false,
		
		Ordering::Less => {
			/* 
				check is violated iff n is between start and x
				hard to understand ...
				it's better to draw it out
				using < or <= ?
				
				we have: 
			   		
			  		0 |----------------S--------X---------------------| (wrapparound)
			  
			   	X is between S and E (S < X < E) in these cases:
			    	0 |----------------S--------X-----E---------------| (wrapparound)
					
					0 |-------------E--S--------X---------------------| (wrapparound)
				
				but *not* in these cases
					0 |----------------S---E----X---------------------| (wrapparound)

					0 |----------------|--------X---------------------| (wrapparound)
									 ^- S+E
					
					0 |----------------S--------|---------------------| (wrapparound)
									     X+E -^									 

				or , in other words, iff !(S <= E <= X)
			*/
			if start <= end && end <= x {
				return false;
			}
		},
		Ordering::Greater => {
			/* 
				check is violated iff n is between start and x
				hard to understand ...
				it's better to draw it out
				using < or <= ?
				
				we have the opposite of above: 
			   		
			  		0 |----------------X--------S---------------------| (wrapparound)
			  
			   	X is between S and E (S < X < E) *only* in these cases:
				   
				   0 |----------------X--E----S-----------------------| (wrapparound)
			    	
				but *not* in these cases
					
					0 |----------------X------S---E-------------------| (wrapparound)
					
					0 |-------------E--X-------S----------------------| (wrapparound)

					0 |---------------|--------S----------------------| (wrapparound)
									  ^- X+E
					
					0 |---------------X--------|----------------------| (wrapparound)
									      S+E -^									 

				or , in other words, iff S < E < X
			*/
			// wrap around because una >= ackn
			// check is Okay iff n is betwween u and a
			if end < start && end > x {
			} else {
				return false;
			}
		}
	}
		
	true
}