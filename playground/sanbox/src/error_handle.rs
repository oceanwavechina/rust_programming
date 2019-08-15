use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;


fn test_match_error() {

   let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("problem opening the file");
        }
    });

   // panic!("crash and burn");
}

/*
    使用 ？ 可以
*/
fn read_username_from_file() -> Result<String, io::Error> {

    return fs::read_to_string("hello.txt");

    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn run() {
    test_match_error();
    read_username_from_file();
}