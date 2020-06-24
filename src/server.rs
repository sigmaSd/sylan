use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use std::net::TcpListener;
type CatchAll<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> std::io::Result<()> {
    let port = std::env::args().nth(1).expect("no port specefied");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
    Ok(())
} // the stream is closed here

fn handle_client(stream: TcpStream) {
    handle_client_(stream).expect("handle_client failed");
}

fn handle_client_(stream: TcpStream) -> CatchAll<()> {
    let mut stream = BufReader::new(stream);
    let mut len = String::new();
    let mut data_type = String::new();
    let mut name = String::new();

    // 1st msg is the name of the file/dir
    stream.read_line(&mut name)?;

    // 2nd msg is the data type (archive or not)
    stream.read_line(&mut data_type)?;

    // 3nd msg is the len of the data
    stream.read_line(&mut len)?;

    let len: usize = len.trim().parse()?;

    let mut buffer = vec![0; len];

    // read data
    let mut i = 0;
    loop {
        match stream.read(&mut buffer[i..]) {
            Ok(0) => break,
            Ok(n) => {
                i += n;
            }
            Err(e) => panic!(e),
        }
    }

    if data_type.trim() == "a" {
        let mut ar = tar::Archive::new(buffer.as_slice());
        ar.unpack(name.trim())?;
    } else {
        let mut out = std::fs::File::create(&name.trim())?;
        out.write_all(&buffer)?;
    }

    Ok(())
}
