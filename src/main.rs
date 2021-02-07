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

    // loop vcards
    let mut count = 0;
    for vcard in vcf.get_vcards() {

        // parse contact from one vcard
        let ct = Contact::new(&vcard);
        if ct.is_empty() { continue; }
        let name = format!("{} - {:?}", ct.name_index(), ct.full_name());
        count += 1;

        // loop telephone
        for telephone in ct.tel_iter() {
            let number = telephone.get_number();
            let tel_type = if telephone.get_type().is_empty() {
                "".to_string()
            } else {
                format!(" ({})", telephone.get_type())
            };
            let xml = Contact::fmt_xml(name.as_ref(), tel_type.as_ref(), number);
            println!("{}", xml);
        }
    }
    println!("all: {}", count);

    /*
    let _filename = "./testfiles/output_test.xml";
    let mut hfile = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(&_filename)
    {
        Ok(h) => h,
        _ => panic!("ファイルが作成できません"),
    };

    if let Err(e) = writeln!(hfile, "A new line!100") {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(hfile, "A new line!101") {
        eprintln!("Couldn't write to file: {}", e);
    }
    */
}
