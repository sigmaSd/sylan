use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let data = prepare_data().expect("prepare_data failed");

    writeln!(stream, "{}", data.name)?;

    if data.archive {
        writeln!(stream, "a")?;
    } else {
        writeln!(stream, "f")?;
    }

    writeln!(stream, "{}", data.len)?;

    stream.write_all(&data.data)?;

    Ok(())
} // the stream is closed here

struct Data {
    len: usize,
    data: Vec<u8>,
    archive: bool,
    name: String,
}

type CatchAll<T> = Result<T, Box<dyn std::error::Error>>;

fn prepare_data() -> CatchAll<Data> {
    let path = std::env::args().nth(1).expect("No target specified");
    let path = std::path::Path::new(&path);

    let name = || -> Option<String> { Some(path.file_name()?.to_str()?.to_string()) }()
        .unwrap_or_else(|| "sylan".to_string());

    if path.is_dir() {
        let tmp = std::env::temp_dir();
        let archive_path = tmp.join(&name);
        let archive_file = std::fs::File::create(&archive_path)?;
        let mut archive = tar::Builder::new(archive_file);
        archive.append_dir_all(".", &path)?;
        archive.finish()?;

        let file = std::fs::read(&archive_path)?;
        Ok(Data {
            len: file.len(),
            data: file,
            archive: true,
            name,
        })
    } else {
        let file = std::fs::read(path)?;
        Ok(Data {
            len: file.len(),
            data: file,
            archive: false,
            name,
        })
    }
}