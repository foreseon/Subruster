use std::{fs,fs::File,io::{self, prelude::*},};

pub struct BufReader {
    reader: io::BufReader<File>,
}

impl BufReader {
    pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        Ok(Self { reader })
    }

    pub fn read_line<'buf>(
        &mut self,
        buffer: &'buf mut String,
    ) -> Option<io::Result<&'buf mut String>> {
        buffer.clear();

        self.reader
            .read_line(buffer)
            .map(|u| if u == 0 { None } else { Some(buffer) })
            .transpose()
    }
}

pub fn write_to_file(filename:String, path: String, content: String) -> std::io::Result<()> {
    
    match create_directory(&path) {
        Ok(n) => {},
        Err(e) => println!("{}", e.to_string())
    }

    let filename = filename.to_owned();
    let mut path = path.to_owned();

    path.push_str(&filename);

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn create_directory (path: &String) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}