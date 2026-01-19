use crate::{Error, Result};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum RgzEntry {
    File {
        name: String,
        data: Vec<u8>,
    },
    Directory {
        name: String,
    },
}

#[derive(Debug)]
pub struct Rgz {
    pub entries: Vec<RgzEntry>,
}

impl Rgz {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let data = std::fs::read(path)?;
        Self::from_bytes(&data)
    }
    
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| Error::Decompression(e.to_string()))?;
        
        let mut entries = Vec::new();
        let mut cursor = std::io::Cursor::new(decompressed);
        
        loop {
            let mut type_buf = [0u8; 1];
            if cursor.read(&mut type_buf).unwrap_or(0) == 0 {
                break;
            }
            let entry_type = type_buf[0];
            
            match entry_type {
                b'f' => {
                    let mut name_len_buf = [0u8; 1];
                    cursor.read_exact(&mut name_len_buf)?;
                    let name_len = name_len_buf[0] as usize;
                    
                    let mut name_buf = vec![0u8; name_len];
                    cursor.read_exact(&mut name_buf)?;
                    let name = String::from_utf8_lossy(&name_buf[..name_len.saturating_sub(1)]).to_string();
                    
                    let mut size_buf = [0u8; 4];
                    cursor.read_exact(&mut size_buf)?;
                    let size = u32::from_le_bytes(size_buf);
                    
                    let mut data = vec![0u8; size as usize];
                    cursor.read_exact(&mut data)?;
                    
                    entries.push(RgzEntry::File { name, data });
                },
                b'd' => {
                    let mut name_len_buf = [0u8; 1];
                    cursor.read_exact(&mut name_len_buf)?;
                    let name_len = name_len_buf[0] as usize;
                    
                    let mut name_buf = vec![0u8; name_len];
                    cursor.read_exact(&mut name_buf)?;
                    let name = String::from_utf8_lossy(&name_buf[..name_len.saturating_sub(1)]).to_string();
                    
                    entries.push(RgzEntry::Directory { name });
                },
                b'e' => break,
                _ => return Err(Error::InvalidRgzFormat),
            }
        }
        
        Ok(Rgz { entries })
    }
    
    pub fn get_entries(&self) -> &[RgzEntry] {
        &self.entries
    }
    
    pub fn new() -> Self {
        Rgz {
            entries: Vec::new(),
        }
    }
    
    pub fn add_file(&mut self, name: &str, data: &[u8]) {
        self.entries.push(RgzEntry::File {
            name: name.to_string(),
            data: data.to_vec(),
        });
    }
    
    pub fn add_directory(&mut self, name: &str) {
        self.entries.push(RgzEntry::Directory {
            name: name.to_string(),
        });
    }
    
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut data = Vec::new();
        
        for entry in &self.entries {
            match entry {
                RgzEntry::File { name, data: file_data } => {
                    if name.len() > 254 {
                        return Err(Error::Custom("Name too long (max 254 bytes)".to_string()));
                    }
                    
                    data.write_all(&[b'f'])?;
                    data.write_all(&[(name.len() + 1) as u8])?;
                    data.write_all(name.as_bytes())?;
                    data.write_all(&[0])?;
                    data.write_all(&(file_data.len() as u32).to_le_bytes())?;
                    data.write_all(file_data)?;
                },
                RgzEntry::Directory { name } => {
                    if name.len() > 254 {
                        return Err(Error::Custom("Name too long (max 254 bytes)".to_string()));
                    }
                    
                    data.write_all(&[b'd'])?;
                    data.write_all(&[(name.len() + 1) as u8])?;
                    data.write_all(name.as_bytes())?;
                    data.write_all(&[0])?;
                },
            }
        }
        
        data.write_all(&[b'e'])?;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&data)?;
        let compressed = encoder.finish()?;
        
        std::fs::write(path, compressed)?;
        
        Ok(())
    }
}
