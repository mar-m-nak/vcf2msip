pub use std::fs::File;
use std::io::{BufReader, Read};
use kanaria::{string::UCSStr, utils::ConvertTarget};
use regex::Regex;

pub const _ERR_FILE_NOT_FOUND:i32 = 1;
pub const _ERR_CREATE_FILE:i32 = 2;
pub const _ERR_WRITE_FILE:i32 = 3;
pub const _ERR_READ_FILE:i32 = 4;

#[derive(Debug)]
pub struct Telephone {
    teltype: String,
    number: String,
}

#[derive(Debug)]
pub struct Contact {
    full_name: String,
    name: String,
    xfirst_name: String,
    xlast_name: String,
    categories: String,
    organization: String,
    tel_numbers: Vec<Telephone>,
}

#[derive(Debug)]
pub struct Vcf {
    data: String,
}

impl Vcf {

    /// load vcf file to buffer
    pub fn new(filename: &str) -> Result<Self, i32> {
        let hfile = match File::open(&filename) {
            Ok(h) => h,
            _ => return Err(_ERR_FILE_NOT_FOUND),
        };
        let mut reader = BufReader::new(&hfile);
        let mut vcf = Self { data: String::with_capacity(1048576) };
        if let Err(_) = reader.read_to_string(&mut vcf.data) {
            return Err(_ERR_READ_FILE);
        }
        Ok(vcf)
    }

    /// return split vcards
    pub fn get_vcards(&self) -> Vec<&str> {
        let mut block: Vec<&str> = self.data.split("END:VCARD").collect();
        block.pop();
        block
    }
}

impl Contact {

    /// parse one vcard
    pub fn new(vcard: &str) -> Self {
        Self {
            full_name: Self::capture("FN:", &vcard),
            name: Self::capture("N:", &vcard),
            xfirst_name: Self::capture("X-PHONETIC-FIRST-NAME:", &vcard),
            xlast_name: Self::capture("X-PHONETIC-LAST-NAME:", &vcard),
            categories: Self::capture("CATEGORIES:", &vcard),
            organization: Self::capture(r"item[\d]*.ORG:", &vcard),
            tel_numbers: Self::cap_tel_numbers(&vcard),
        }
    }

    /// capture value from header line
    fn capture(header: &str, vcard: &str) -> String {
        let pat = format!("(?m)^{}(.*)$", &header);
        let re = Regex::new(&pat).unwrap();
        let raw_value = match re.captures(&vcard) {
            None => "".to_string(),
            Some(s) => s[1].trim_end().to_string(),
        };
        // convert japanese HANKAKU KATAKANA to ZENKAKU
        UCSStr::from_str(&raw_value).wide(ConvertTarget::KATAKANA).to_string()
    }

    /// capture telephones type and number
    fn cap_tel_numbers(vcard: &str) -> Vec<Telephone> {
        let re = Regex::new(r"(?m)^[item\d\.]*TEL(;TYPE=([a-zA-Z]*))*:(.*)$").unwrap();
        let mut vec_telnums:Vec<Telephone> = Vec::new();
        for cap in re.captures_iter(&vcard) {
            let teltype = cap.get(2).map_or("", |m| m.as_str());
            let number = cap.get(3).map_or("", |m| m.as_str());
            if number != "" { vec_telnums.push(
                Telephone{
                    teltype: teltype.to_string(),
                    number: number.trim_end().to_string(),
                }
            ); }
        }
        vec_telnums
    }

    /// is no telephone number
    pub fn is_empty(&self) -> bool {
        self.tel_numbers.is_empty()
    }

    /// return full or organization name
    pub fn full_name(&self) -> &str {
        if !self.full_name.is_empty() {
            self.full_name.as_ref()
        } else {
            self.organization.as_ref()
        }
    }

    /// return initial from last or full or org name
    pub fn name_index(&self) -> String {
        let target_name = if !self.xlast_name.is_empty() {
            &self.xlast_name
        } else if !self.full_name.is_empty() {
            &self.full_name
        } else {
            &self.organization
        };
        let hira = UCSStr::from_str(&target_name).hiragana().to_string();
        let v:Vec<&str> = hira.split("").collect();
        match v.get(1) {
            Some(s) => s.to_string(),
            _ => "".to_string()
        }
    }

    pub fn categories(&self) -> &str {
        self.categories.as_ref()
    }

    /// return telephones iterator
    pub fn tel_iter(&self) -> impl Iterator<Item = &Telephone> {
        self.tel_numbers.iter()
    }

    /// return formated one xml line
    pub fn fmt_xml(name: &str, tel_type: &str, number: &str) -> String {
        let xml = r#"<contact name="%name%%type%" number="%number%" firstname="" lastname="" phone="" mobile="" email="" address="" city="" state="" zip="" comment="" id="" info="" presence="0" directory="0"/>"#;
        xml .replace("%name%", name)
            .replace("%type%", tel_type)
            .replace("%number%", number)
    }
}

impl Telephone {
    pub fn get_type(&self) -> &str {
        self.teltype.as_ref()
    }
    pub fn get_number(&self) -> &str {
        self.number.as_ref()
    }
}

