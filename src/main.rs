mod vcf_parser;

use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use std::process::exit;
use vcf_parser::*;

fn main() {
    // open and read vcf file
    let _filename = "./testfiles/contacts.vcf";
    let vcf = match Vcf::new(&_filename) {
        Ok(vcf) => vcf,
        Err(e) => { print_err_msg(e); exit(e); }
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
        _ => { print_err_msg(_ERR_CREATE_FILE); exit(_ERR_CREATE_FILE); }
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

/// print my error message
fn print_err_msg(e: i32) {
    let msg = match e {
        _ERR_FILE_NOT_FOUND => "ファイルが見つかりません",
        _ERR_CREATE_FILE => "ファイル作成に失敗しました",
        _ERR_READ_FILE => "ファイル読み込みに失敗しました",
        _ERR_WRITE_FILE => "ファイル書き込みに失敗しました",
        _ => "",
    };
    if !msg.is_empty() { println!("{}", msg) } else { println!("不明なエラーです {}", e) };
}
