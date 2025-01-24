use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Seek, Read};
use std::path::Path;

// This code processes lots of Vec<u8> data. Because that’s used in the
// same way as String tends to be used, ByteString is a useful alias.
type ByteString = Vec<u8>;
// ByteStr is to &str what ByteString is to Vec<u8>.
type ByteStr = [u8];

// Instructs the compiler to generate serialized code to enable writing
// KeyValuePair data to disk. 
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
    f: File,
    // Maintains a mapping between keys and file locations.
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let f= OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::new();

        Ok(ActionKV { 
            f,
            index,
        })
    }

    pub fn process_record<R: Read>(f: &mut R) -> std::io::Result<KeyValuePair> {
        Ok(KeyValuePair {
            key: vec![],
            value: vec![],
        })       
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            let position = f.seek(std::io::SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut f);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        std::io::ErrorKind::UnexpectedEof => {
                            break;
                        },
                        _ => return Err(err),
                    }
                }
            };

            self.index.insert(kv.key, position);
        }

        Ok(())
    }
}
