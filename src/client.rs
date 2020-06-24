use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let ip = args.next().expect("Ip was not specified");
    let port = args.next().expect("Port was not specified");
    let path = args.next().expect("No target specified");

    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;

    let data = prepare_data(&path).expect("prepare_data failed");

    writeln!(stream, "{}", data.name)?;

    if data.archive {
        writeln!(stream, "a")?;
    } else {
        writeln!(stream, "f")?;
    }

    let mut buffer = [0; 1000];

    let mut file = std::fs::File::open(data.file_path)?;

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                stream.write_all(&buffer[..n])?;
            }
            Err(e) => panic!(e),
        }
    }

    // clean up
    if data.archive {
        std::fs::remove_file(format!("{}.tar", data.name))?;
    }

    //    stream.write_all(&data.file_path)?;

    Ok(())
} // the stream is closed here

struct Data {
    file_path: std::path::PathBuf,
    archive: bool,
    name: String,
}

type CatchAll<T> = Result<T, Box<dyn std::error::Error>>;

fn prepare_data(path: &str) -> CatchAll<Data> {
    let path = std::path::Path::new(&path);

    let name = || -> Option<String> { Some(path.file_name()?.to_str()?.to_string()) }()
        .unwrap_or_else(|| "sylan".to_string());

    if path.is_dir() {
        let archive_name = format!("{}.tar", name);
        let archive_path = std::path::Path::new(&archive_name);
        let archive_file = std::fs::File::create(&archive_path)?;
        let mut archive = tar::Builder::new(archive_file);
        archive.append_dir_all(".", &path)?;
        archive.finish()?;

        Ok(Data {
            file_path: archive_path.to_path_buf(),
            archive: true,
            name,
        })
    } else {
        Ok(Data {
            file_path: path.to_path_buf(),
            archive: false,
            name,
        })
    }
}
