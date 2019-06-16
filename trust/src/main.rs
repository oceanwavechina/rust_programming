use std::{io, thread};
use std::io::prelude::*;


fn main() -> io::Result<()> {
	let mut i = trust::Interface::new()?;
	let mut l = i.bind(9000)?;
	let jh = thread::spawn(move || {
		while let Ok(mut stream) = l.accept(){
			eprintln!("got connection on 9000 !");
			let n = stream.read(&mut [0]).unwrap();
			eprintln!("read data");
			assert_eq!(n, 0);
			eprintln!("no more data");
			
		}
	});
	jh.join().unwrap();
	Ok(())
}
