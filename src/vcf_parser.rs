use crate::error_flg;
use crate::arg_and_help;

pub use std::fs::File;
pub use std::io::{BufReader, Read};
use kanaria::{string::UCSStr, utils::ConvertTarget};
pub use regex::Regex;
use error_flg::*;
use arg_and_help::{ARG_PAT_NAME, ARG_PAT_INITIAL, ARG_PAT_CATEGORIES, ARG_PAT_TEL_TYPE};

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

    /// Read vcf file
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

    /// Return split vcards
    pub fn get_vcards(&self) -> Vec<&str> {
        let mut block: Vec<&str> = self.data.split("END:VCARD").collect();
        block.pop();
        block
    }
}

impl Contact {

    /// Parse one vcard
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

    /// Capture value from prefixed line
    fn capture(prefix: &str, vcard: &str) -> String {
        let pat = format!("(?m)^{}(.*)$", &prefix);
        let re = Regex::new(&pat).unwrap();
        let raw_value = match re.captures(&vcard) {
            None => "".to_string(),
            Some(s) => s[1].trim_end().to_string(),
        };
        // Convert japanese HANKAKU KATAKANA to ZENKAKU
        UCSStr::from_str(&raw_value).wide(ConvertTarget::KATAKANA).to_string()
    }

    /// Capture telephones type and number
    fn cap_tel_numbers(vcard: &str) -> Vec<Telephone> {
        let re = Regex::new(r"(?m)^(item\d)*[\.]*TEL(;TYPE=([a-zA-Z]*))*:(.*)$").unwrap();
        let mut vec_telnums:Vec<Telephone> = Vec::new();
        for cap in re.captures_iter(&vcard) {
            let number = cap.get(4).map_or("", |m| m.as_str());
            let mut teltype = cap.get(3).map_or("", |m| m.as_str()).to_string();
            if teltype.is_empty() {
                let item = cap.get(1).map_or("", |m| m.as_str());
                teltype = Self::find_item_label(&vcard, item);
            }
            if number != "" { vec_telnums.push(
                Telephone{
                    teltype: teltype,
                    number: number.trim_end().to_string(),
                }
            ); }
        }
        vec_telnums
    }

    /// Find X-ABLabel for against item
    fn find_item_label(vcard: &str, item: &str) -> String {
        let pat = format!(r"(?m)^{}.X-ABLabel:(.*)$", item);
        let re = Regex::new(&pat).unwrap();
        let res = match re.captures(&vcard) {
            Some(cap) => cap.get(1).map_or("", |m| m.as_str()),
            None => "",
        };
        res.trim_end().to_string()
    }

    /// Check telephone number is empty
    pub fn is_empty(&self) -> bool {
        self.tel_numbers.is_empty()
    }

    /// Return full name, or organization name
    fn full_name(&self) -> &str {
        if !self.full_name.is_empty() {
            self.full_name.as_ref()
        } else {
            self.organization.as_ref()
        }
    }

    /// Return initial from last or full or org name
    // TODO: 苗字と名前
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

    /// Return telephones iterator
    pub fn tel_iter(&self) -> impl Iterator<Item = &Telephone> {
        self.tel_numbers.iter()
    }

    /// Return one xml element line
    pub fn xml_line(name: &str, number: &str) -> String {
        let line = r#"<contact name="%name%" number="%number%" firstname="" lastname="" phone="" mobile="" email="" address="" city="" state="" zip="" comment="" id="" info="" presence="0" directory="0"/>"#;
        line.replace("%name%", name)
            .replace("%number%", number)
    }

    /// Return formated name from pattern
    pub fn fmt_name(&self, name_pattern: &str, initial: &str, teltype: &str) -> String {
        name_pattern
            .replace(ARG_PAT_NAME, &self.full_name())
            .replace(ARG_PAT_TEL_TYPE, teltype)
            .replace(ARG_PAT_CATEGORIES, &self.categories)
            .replace(ARG_PAT_INITIAL, initial)
            .replace("()", "")
            .replace("[]", "")
            .trim().to_string()
    }
}

impl Telephone {
    pub fn teltype(&self) -> &str {
        self.teltype.as_ref()
    }
    pub fn number(&self) -> &str {
        self.number.as_ref()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_VCF_FILENAME: &'static str = r".\testfiles\test.vcf";

    #[test]
    fn test_vcf_parse() {
        let vcf = Vcf::new(TEST_VCF_FILENAME).unwrap();
        let vcs = vcf.get_vcards();
        let ct = Contact::new(&vcs[0]);
        assert_eq!("Taro Yamada", ct.full_name());
        let tels  = ct.tel_numbers;
        assert_eq!("1234", tels[1].number);
        assert_eq!("WORK", tels[1].teltype);
        assert_eq!("11-22-33", tels[2].number);
        assert_eq!("", tels[2].teltype);
        assert_eq!("55-66-77", tels[3].number);
        assert_eq!("homeFax", tels[3].teltype);
        let ct = Contact::new(&vcs[1]);
        assert_eq!("太宰治", ct.full_name());
        assert_eq!("だ", ct.name_index());
        let ct = Contact::new(&vcs[2]);
        assert_eq!("CORPCORP", ct.full_name());
        assert_eq!("Business", ct.categories);
    }
}
