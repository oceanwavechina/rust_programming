use std::{io, thread};
use std::io::prelude::*;


fn main() -> io::Result<()> {
	let mut i = trust::Interface::new()?;
	let mut l = i.bind(9000)?;
	let jh = thread::spawn(move || {
		while let Ok(mut stream) = l.accept(){
			eprintln!("got connection on 9000 !");
			loop {
				let mut buf = [0; 512];
				let n = stream.read(&mut buf[..]).unwrap();
				eprintln!("read {}b of data", n);
				if n == 0 {
					eprintln!("no more data");
					break;
				} else {
					println!("got {:?}", &buf[..n]);
				}
			}
			
		}
	});
	jh.join().unwrap();
	Ok(())
}
