use crate::vcf_parser;

use vcf_parser::*;
// use widestring::U16String;

#[derive(Debug)]
pub struct IniIo {
    data: String,
}

impl IniIo {
    /// load .ini file (utf16bom) to string
    pub fn new(filename: &str) -> Result<Self, i32> {
        let hfile = match File::open(&filename) {
            Ok(h) => h,
            _ => return Err(_ERR_FILE_NOT_FOUND),
        };
        let mut reader = BufReader::new(&hfile);
        let mut ini_byte: Vec<u8> = Vec::new();
        if let Err(_) = reader.read_to_end(&mut ini_byte) {
            return Err(_ERR_READ_FILE);
        }
        let ini_vec_u16: Vec<u16> = ini_byte
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
            .collect();
        // convert to utf8 string
        let ini_slice = ini_vec_u16.as_slice();
        let data = String::from_utf16_lossy(ini_slice);
        // or use U16String crate
        // let wstr = U16String::from_vec(ini_vec_u16);
        // let data = wstr.to_string().unwrap();
        Ok(Self { data })
    }
}
