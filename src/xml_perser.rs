use std::io::BufRead;

use crate::error_flg;
use crate::vcf_parser;

use error_flg::*;
use vcf_parser::*;

/// Existing original sip contacts
#[derive(Debug)]
pub struct SipContacts {
    data: Vec<(String, String, String)>, // number(numeric only), number(original), name
}

impl SipContacts {

    /// Return empty for no merge mode
    pub fn empty() -> Self {
        let sip_vec: Vec<(String, String, String)> = Vec::new();
        Self{ data: sip_vec }
    }

    /// Return original sip contact
    pub fn new(filename: &str) -> Result<Self, i32> {
        let hfile = match File::open(&filename) {
            Ok(h) => h,
            _ => return Err(_ERR_FILE_NOT_FOUND),
        };
        let mut reader = BufReader::new(&hfile);
        let mut sip_vec: Vec<(String, String, String)> = Vec::new();
        let mut buf = String::with_capacity(512);
        let pat = "<contact name=\"(.*)\" number=\"(.*)\" firstname=.*";
        let re = Regex::new(&pat).unwrap();
        loop {
            let len = match reader.read_line(&mut buf) {
                Ok(l) => l,
                Err(_) => 0,
            };
            if len == 0 { break; }
            if let Some(c) = re.captures(&buf) {
                sip_vec.push( (fix_number(&c[2]), c[2].to_string(), c[1].to_string()) );
            }
            buf.clear();
        }
        Ok( Self{ data: sip_vec } )
    }

    /// Data getter
    pub fn data(&self) -> &Vec<(String, String, String)> {
        &self.data.as_ref()
    }

    /// Empty is no merge
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear buffer in sip contact, if phone number exist
    pub fn clear_exist(&mut self, number: &str) {
        let mut idx = 0;
        let mut clr_idx: Vec<usize> = Vec::new();
        for cts in &self.data {
            if  cts.0 == fix_number(number) {
                clr_idx.push(idx);
            };
            idx += 1;
        }
        for idx in clr_idx {
            self.data[idx].0.clear();
            self.data[idx].1.clear();
            self.data[idx].2.clear();
        }
    }
}