mod file;

use crate::file::File;
use crate::file::read_lines;

pub struct DesktopEntry {
    pub file: String,
    pub header: String,
    pub key: String
}

impl DesktopEntry {
    pub fn new(file: String, header: String, key: String) -> Self {
        Self {
            file,
            header,
            key
        }
    }

    pub fn getvalue(&self) -> String {
        if self.file.isfile() {
            let value = String::new();
            let flines: Vec<String> = read_lines(&self.file);
            let mut inscope: bool = false;

            for ctx in flines {
                if inscope {
                    // yes, header is exists, now searching for the key under header.
                    println!("{0}", ctx);
                } else {
                    // no, header can't find or already passed so the key is not exists.
                    println!("{0}", ctx);
                }
            }

            value
        } else {
            String::new()
        }
    }
}