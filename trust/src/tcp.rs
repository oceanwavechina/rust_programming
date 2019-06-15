use std::io;
use std::collections::VecDeque;
use std::io;


pub enum State {
	// Listen,
	SynRcvd,
	Estab,
	FinWait1,
	FinWait2,
	TimeWait,
}

impl State {
	fn is_non_synchronized(&self) -> bool {
		match *self {
			State::SynRcvd => false,
			State::Estab | State::FinWait1 | State::FinWait2 | State::TimeWait => true,
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

	incoming: VecDeque<u8>,
	unacked: VecDeque<u8>,
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
			// only expected SYN packet
			return Ok(None);
		}

		// 初始化Connection结构体
		let iss = 0;		// initial sequence number
		let wnd = 10;		// window
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
				wl2: 0,
			},
			recv: RecvSequenceSpace {
				// keep track of sender info
				irs: tcph.sequence_number(),
				nxt: tcph.sequence_number() +1,
				wnd: tcph.window_size(),
				up: false,
			},
			tcp: etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), iss, wnd),

			//param (payload_len: u16, time_to_live: u8, protocol: IpTrafficClass, source: [u8;4], destination: [u8;4])
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
		// set the syn and ack field
		c.tcp.syn = true;
		c.tcp.ack = true;
		c.write(nic, &[])?;

		Ok(Some(c))
	}

	fn write(&mut self, nic:&mut tun_tap::Iface, payload: &[u8]) -> io::Result<usize> {
		let mut buf =  [0u8; 1500];
		self.tcp.sequence_number = self.send.nxt;
		self.tcp.acknowledgment_number = self.recv.nxt;

		let size = std::cmp::min(
			buf.len(), 
			self.tcp.header_len() as usize + self.ip.header_len() as usize + payload.len()
		);

		self.ip.set_payload_len(size - self.ip.header_len() as usize);

		// kernel does this already
		self.tcp.checksum = self.tcp
				.calc_checksum_ipv4(&self.ip, &[])
				.expect("failed to compute checksum");

		// write out the headers
		use std::io::Write;
		let mut unwritten = &mut buf[..];	
		self.ip.write(& mut unwritten);		// move to next writefull point we have not written yet
		self.tcp.write(& mut unwritten);
		let payload_bytes = unwritten.write(payload)?;
		let unwritten = unwritten.len();
		self.send.nxt = self.send.nxt.wrapping_add(payload_bytes as u32);
		if self.tcp.syn {
			self.send.nxt = self.send.nxt.wrapping_add(1);
			self.tcp.syn = false;
		}
		if self.tcp.fin {
			self.send.nxt = self.send.nxt.wrapping_add(1);
			self.tcp.fin = false;
		}

		nic.send(&buf[..buf.len() - unwritten])?;
		Ok(payload_bytes)
	}

	fn send_rst( &mut self, nic: &mut tun_tap::Iface, ) -> io::Result< () > {
		self.tcp.rst = true;

		// TODO: fix sequcence number here
		
		// If the incoming segment has an ACK field, the reset takes its
		// sequence number from the ACK field of the segment, otherwise the
		// reset has sequence number zero and the ACK field is set to the sum
		// of the sequence number and segment length of the incoming segment.
		// The connection remains in the CLOSED state.
		
		// TODO: handle synchronized RST
		// 3.  If the connection is in a synchronized state (ESTABLISHED,
		// FIN-WAIT-1, FIN-WAIT-2, CLOSE-WAIT, CLOSING, LAST-ACK, TIME-WAIT),
		// any unacceptable segment (out of window sequence number or
		// unacceptible acknowledgment number) must elicit only an empty
		// acknowledgment segment containing the current send-sequence number
		// and an acknowledgment indicating the next sequence number expected
		// to be received, and the connection remains in the same state.

		self.tcp.sequence_number = 0;
		self.tcp.acknowledgment_number = 0;
		self.write(nic, &[])?;
		Ok(())
	}

	pub fn on_packet<'a> (
		&mut self,
		nic: &mut tun_tap::Iface,
		iph: etherparse::Ipv4HeaderSlice<'a>, 
		tcph: etherparse::TcpHeaderSlice<'a>, 
		data: &'a [u8]
	) -> io::Result<()> {
		//fist, check that sequence numbers are valid (RFC S3.3)
		let seqn = tcph.sequence_number();
		let mut slen = data.len() as u32;
		if tcph.fin() {
			slen += 1;
		};
		if tcph.syn() {
			slen += 1;
		};
		let wend = self.recv.nxt.wrapping_add(self.recv.wnd as u32);
		let okay = if slen == 0 {
			// zero length segment has  sperate rules for acceptance
			if self.recv.wnd == 0 {
				if seqn != self.recv.nxt {
					false
				} else {
					true
				}
			} else if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn, wend) {
				false
			} else {
				true
			}
		} else {
			if self.recv.wnd == 0 {
				false
			} else if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn, wend) &&
					!is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn.wrapping_add(slen - 1), wend) 
			{
				false
			} else {
				true
			}
		};

		if !okay {
			self.write(nic, &[])?;
			return Ok(());
		}

		self.recv.nxt = seqn.wrapping_add(slen);
		// TODO: if _not_ acceptable , send ACK
		// <SEQ=SND.NXT><ACK=RCV.NXT><CTL=ACK>

		if !tcph.ack() {
			return Ok(());
		}

		let ackn = tcph.acknowledgment_number();
		if let State::SynRcvd = self.state {
			if is_between_wrapped(self.send.una.wrapping_sub(1), ackn, self.send.nxt.wrapping_add(1)) {
				// must have  ACKed our SYN, since we detected at least one acked byte
				// and we have only sent one byte (the SYN)
				// the three-way handshake finished !!!
				self.state = State::Estab;
			} else {
				// TODO: <SEQ=SEG.ACK><CTL=RST>
			}
		}

		if let State::Estab | State::FinWait1 | State::FinWait2  = self.state {
			if !is_between_wrapped(self.send.una, ackn, self.send.nxt.wrapping_add(1)) {
				return Ok(());
			}
			self.send.una = ackn;
			// TODO
			assert!(data.is_empty());

			if let State::Estab = self.state {
				// now let's terminate the connection!
				// TODO: needs to be stored in the retransmission queue!
				self.tcp.fin = true;
				self.write(nic, &[])?;
				self.state = State::FinWait1;
			}
		}

		if let State::FinWait1 = self.state {
			if self.send.una == self.send.iss + 2 {
				// our FIN has been acked!
				self.state = State::FinWait2;
			}
		}

		if tcph.fin() {
			match self.state {
				State::FinWait2 => {
					// we're done with the connection !
					self.write(nic, &[])?;
					self.state = State::TimeWait;
				}
				_ => unimplemented!(),
			}
		}

		Ok(())
	}
}

fn wrapping_it(lhs:u32, rhs: u32) -> bool {

	// RFC1323:
	// 	 TCP determines if a data segment is "old" or "new" by testing
    //   whether its sequence number is within 2**31 bytes of the left edge
    //   of the window, and if it is not, discarding the data as "old".  To
    //   insure that new data is never mistakenly considered old and vice-
    //   versa, the left edge of the sender's window has to be at most
    //   2**31 away from the right edge of the receiver's window.

	lhs.wrapping_sub(rhs) > 2^31;
}

fn is_between_wrapped(start: u32, x: u32, end:u32) -> bool {
	wrapping_it(start, x) && wrapping_it(x, end)
}