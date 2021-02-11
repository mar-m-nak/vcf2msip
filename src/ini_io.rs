use std::io::{BufWriter, Write};

use crate::vcf_parser;

use vcf_parser::*;
// use widestring::U16String;

pub const _ERR_DID_NOT_RUN_RENEW_LOGS:i32 = 101;
pub const _ERR_WRITE_INI_FILE:i32 = 102;

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

    /// return line string in match number from ini
    pub fn get_match_number_line(&self, number: &str) -> String {
        let mut fix_number = String::with_capacity(16);
        for c in number.chars() {
            if c.is_numeric() {
                fix_number = format!("{}{}", fix_number, c);
            }
        };
        let pat = format!(r"(?m)^\d={};.*$", fix_number);
        let re = Regex::new(&pat).unwrap();
        let hit_line = match re.captures(&self.data) {
            None => { return  "".to_string() },
            Some(s) => { s[0].trim_end().to_string() },
        };
        hit_line
    }

    /// return renewed number line
    pub fn make_new_number_line(old_line: &str, new_name: &str) -> String {
        let v: Vec<&str> = old_line.split(';').collect();
        if v.len() != 6 { return "".to_string(); };
        // let mut new_line = String::with_capacity(old_line.len() + new_name.len());
        let fix_new_name = new_name.replace(";", "");
        let new_line = format!(
            "{};{};{};{};{};{}", v[0], fix_new_name, v[2], v[3], v[4], v[5],
        );
        new_line
    }

    /// replace ini string
    pub fn replace(&mut self, old_line: &str, new_line: &str) {
        self.data = self.data.replace(old_line, new_line);
    }

    /// save ini file
    pub fn save(&self) -> Result<(), i32> {
        let filename = "./testfiles/MicroSIP_new.ini";
        let hfile = match File::create(&filename) {
            Ok(h) => h,
            _ => return Err(_ERR_FILE_NOT_FOUND),
        };
        let mut writer = BufWriter::new(&hfile);
        if let Err(_) = writer.write_all(self.data.as_bytes()) {
            return Err(_ERR_WRITE_INI_FILE);
        };
        Ok(())
    }

}
