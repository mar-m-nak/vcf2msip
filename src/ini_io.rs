use crate::error_flg;
use crate::vcf_parser;

use std::io::{BufWriter};
use error_flg::*;
use vcf_parser::*;
use file_utils::write::Write as fu_write;

#[derive(Debug)]
pub struct IniIo {
    data: String,
}

impl IniIo {
    /// Read .ini file (utf16bom) to string
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
        Ok(Self { data })
    }

    /// Return line string in match number from ini
    pub fn get_match_number_line(&self, number: &str) -> String {
        let pat = format!(r"(?m)^\d={};.*$", fix_number(&number));
        let re = Regex::new(&pat).unwrap();
        let hit_line = match re.captures(&self.data) {
            None => { return  "".to_string() },
            Some(s) => { s[0].trim_end().to_string() },
        };
        hit_line
    }

    /// Return renewed line string
    pub fn make_new_number_line(old_line: &str, new_name: &str) -> String {
        let v: Vec<&str> = old_line.split(';').collect();
        if v.len() != 6 { return "".to_string(); };
        let fix_new_name = new_name.replace(";", "");
        let new_line = format!(
            "{};{};{};{};{};{}", v[0], fix_new_name, v[2], v[3], v[4], v[5],
        );
        new_line
    }

    /// Replace ini string
    pub fn replace(&mut self, old_line: &str, new_line: &str) {
        self.data = self.data.replace(old_line, new_line);
    }

    /// Save ini file
    pub fn save(&self, filename: &str) -> Result<(), i32> {
        let hfile = match File::create(&filename) {
            Ok(h ) => h,
            _ => return Err(_ERR_FILE_NOT_FOUND),
        };
        let mut writer = BufWriter::new(&hfile);
        // Write Utf16 encoded ini buffer
        let vec_u16: Vec<u16> = self.data.encode_utf16().collect();
        for uu in &vec_u16 {
            if let Err(_) = writer.write_u16(*uu) {
                return Err(_ERR_WRITE_INI_FILE);
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INI_FILENAME: &'static str = r".\testfiles\test.ini";
    const TEST_STRING: &'static [&'static str]  = &[
        "\u{feff}[Calls]\r\n0=112233;AAA;2;1111;0;cancel\r\n1=445566;BBB;2;2222;0;cancel\r\n2=778899;CCC;2;3333;0;cancel\r\n",
        "\u{feff}[Calls]\r\n0=112233;AAA;2;1111;0;cancel\r\n1=445566;ZZZ;2;2222;0;cancel\r\n2=778899;CCC;2;3333;0;cancel\r\n"
    ];
    const TEST_LINE: &'static [&'static str] = &[
        "1=445566;BBB;2;2222;0;cancel",
        "1=445566;ZZZ;2;2222;0;cancel"
    ];

    fn test_switch(str: &str) -> usize {
        if let None = str.find("ZZZ") {0} else {1}
    }

    #[test]
    fn test_ini_read() {
        let ini = IniIo::new(TEST_INI_FILENAME).unwrap();
        let sw = test_switch(&ini.data);
        assert_eq!(TEST_STRING[sw], ini.data);
    }

    #[test]
    fn test_ini_get_line() {
        let ini = IniIo::new(TEST_INI_FILENAME).unwrap();
        let sw = test_switch(&ini.data);
        let old_line = ini.get_match_number_line("445566");
        assert_eq!(TEST_LINE[sw], old_line);
    }

    #[test]
    fn test_ini_write_and_read() {
        let mut ini = IniIo::new(TEST_INI_FILENAME).unwrap();
        let sw = test_switch(&ini.data);
        let old_line = ini.get_match_number_line("445566");
        let new_line = if sw == 0 {
            IniIo::make_new_number_line(&old_line, "ZZZ")
        } else {
            IniIo::make_new_number_line(&old_line, "BBB")
        };
        let invsw = (sw as i32 * -1 + 1) as usize;
        assert_eq!(TEST_LINE[invsw], new_line);

        ini.replace(&old_line, &new_line);
        assert_eq!(TEST_STRING[invsw], ini.data);

        ini.save(TEST_INI_FILENAME).unwrap();
        test_ini_read(); // assert renewed file
    }
}