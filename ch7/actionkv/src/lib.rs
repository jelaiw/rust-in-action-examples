use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;
use crc::crc32;

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

    // Assumes that f is already at the right place in the file.
    pub fn process_record<R: Read>(f: &mut R) -> std::io::Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let value_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + value_len;

        let mut data = ByteString::with_capacity(data_len as usize);

        {
            // f.by_ref() is required because .take(n) creates a new Read instance. Using
            // a reference within this block allows us to sidestep ownership issues.
            f.by_ref().take(data_len as u64).read_to_end(&mut data)?;
        }
        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!("data corruption encountered ({:08x} != {:08x})", checksum, saved_checksum);
        }

        let val = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair {
            key: key,
            value: val,
        })       
    }

    pub fn seek_to_end(&mut self) -> std::io::Result<u64> {
        self.f.seek(SeekFrom::End(0))
    }

    // ActionKV::load() populates the index of the ActionKV struct, mapping keys to file positions.
    pub fn load(&mut self) -> std::io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            // File::seek() returns the number of bytes from the start of the file. This becomes the value of the index.
            let position = f.seek(std::io::SeekFrom::Current(0))?;
            // ActionKV::process_record() reads a record in the file at its current position.
            let maybe_kv = ActionKV::process_record(&mut f);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        // Unexpected is relative. The application might not have expected to encounter the end of
                        // the file, but we expect files to be finite, so we’ll deal with that eventuality.
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

    // Wraps Option within Result to allow for the possibility of an I/O error as well as tolerating missing values.
    pub fn get(&mut self, key: &ByteStr) -> std::io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position
        };

        let kv = self.get_at(position)?;
        Ok(Some(kv.value))
    }

    pub fn get_at(&mut self, position: u64) -> std::io::Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut f)?;
        Ok(kv)
    }

    pub fn find(&mut self, target: &ByteStr) -> std::io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.f);
        let mut found: Option<(u64, ByteString)> = None;

        loop {
            let position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        std::io::ErrorKind::UnexpectedEof => {
                            break;
                        }
                        _ => return Err(err),
                    }
                }
            };

            if kv.key == target {
                found = Some((position, kv.value));
            }

            // important to keep looping until the end of the file,
            // in case the key has been overwritten
        }

        Ok(found)
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> std::io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;
        // key.to_vec() converts the &ByteStr to a ByteString.
        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> std::io::Result<u64> {
        // The std::io::BufWriter type batches multiple short write() calls into fewer actual disk operations, resulting in a single one.
        // This increases throughput while keeping the application code neater.
        let mut f = BufWriter::new(&mut self.f);

        let key_len = key.len();
        let val_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + val_len);

        // Iterating through one collection to populate another is slightly awkward, but gets the job done.
        for byte in key {
            tmp.push(*byte);
        }

        for byte in value {
            tmp.push(*byte);
        }

        let checksum = crc32::checksum_ieee(&tmp);

        let next_byte = SeekFrom::End(0);
        let current_position = f.seek(SeekFrom::Current(0))?;
        f.seek(next_byte)?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        f.write_all(&mut tmp)?;

        Ok(current_position)
    }

    #[inline]
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> std::io::Result<()> {
        self.insert(key, value)
    }

    #[inline]
    pub fn delete(&mut self, key: &ByteStr) -> std::io::Result<()> {
        self.insert(key, b"")
    }
}
