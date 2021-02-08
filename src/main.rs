mod vcf_parser;

use std::fs::OpenOptions;
use std::io::prelude::*;
use vcf_parser::{Contact, Vcf}; //, File};

fn main() {
    // open and read vcf file
    let _filename = "./testfiles/contacts.vcf";
    let vcf = match Vcf::new(&_filename) {
        Ok(vcf) => vcf,
        _ => panic!("ファイルが開けません"),
    };

    // create micro-sip xml file
    let _filename = "./testfiles/output_test.xml";
    let mut hxmlfile = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(&_filename)
    {
        Ok(h) => h,
        _ => panic!("ファイルが作成できません"),
    };
    if let Err(e) = writeln!(hxmlfile, "<?xml version=\"1.0\"?>\r\n<contacts>\r") {
        eprintln!("Couldn't write to file: {}", e);
    }

    // loop at vcards
    let mut count_contact = 0;
    let mut count_number = 0;
    for vcard in vcf.get_vcards() {
        // parse one contact
        let ct = Contact::new(&vcard);
        if ct.is_empty() { continue; }
        let name_str =
            format!("{} - {}", ct.name_index(), ct.full_name())
            .replace("\"", "");
        let name: &str = name_str.as_ref();
        // loop at telephone in this contact
        for tel in ct.tel_iter() {
            let number = tel.get_number();
            let tel_type_str = if tel.get_type().is_empty() {
                "".to_string()
            } else {
                format!(" ({})", tel.get_type())
            };
            let tel_type: &str = tel_type_str.as_ref();
            // write to xml file
            let xml = Contact::fmt_xml(name, tel_type, number);
            // println!("{}", xml);
            if let Err(e) = writeln!(hxmlfile, "{}\r", xml) {
                eprintln!("Couldn't write to file: {}", e);
                panic!("書き込み失敗");
            }
            count_number += 1;
        }
        count_contact += 1;
    }
    println!("contact: {} / number: {}", count_contact, count_number);

    if let Err(e) = writeln!(hxmlfile, "</contacts>\r") {
        eprintln!("Couldn't write to file: {}", e);
    }


}
