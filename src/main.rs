mod vcf_parser;

use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use std::process::exit;
use vcf_parser::*;

fn main() {
    if let Err(e) = conv() {
        print_err_msg(e);
        exit(e);
    };
}

fn conv() -> Result<(), i32> {

    // open and read vcf file
    let _filename = "./testfiles/contacts.vcf";
    let vcf = match Vcf::new(&_filename) {
        Ok(vcf) => vcf,
        Err(e) => { return Err(e); },
    };

    // create micro-sip xml file
    let _filename = "./testfiles/output_test.xml";
    let mut hxmlfile = match create_xml_file(&_filename, false) {
        Ok(h) => h,
        Err(e) => { return Err(e); }
    };
    if let Err(_) = writeln!(hxmlfile, "<?xml version=\"1.0\"?>\r\n<contacts>\r") {
        return Err(_ERR_WRITE_FILE);
    }

    // loop at vcards
    let mut count_contact = 0;
    let mut count_number = 0;
    for vcard in vcf.get_vcards() {
        // parse one contact
        let ct = Contact::new(&vcard);
        if ct.is_empty() { continue; }
        let name = format!("{} - {}", ct.name_index(), ct.full_name())
            .replace("\"", "");

        // loop at telephone in this contact
        for tel in ct.tel_iter() {
            let number = tel.get_number();
            let tel_type = if tel.get_type().is_empty() {
                "".to_string()
            } else {
                format!(" ({})", tel.get_type())
            };
            // write to xml file
            let xml = Contact::fmt_xml(name.as_ref(), tel_type.as_ref(), number);
            if let Err(_) = writeln!(hxmlfile, "{}\r", xml) {
                continue;
            }
            count_number += 1;
        }
        count_contact += 1;
    }
    println!("contact: {} / number: {}", count_contact, count_number);

    if let Err(_) = writeln!(hxmlfile, "</contacts>\r") {
        return Err(_ERR_WRITE_FILE);
    }

    Ok(())
}

/// touch output xml file with overwrite or append
fn create_xml_file(filename: &str, is_append: bool) -> Result<File, i32> {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .append(is_append)
        .truncate(!is_append)
        .open(&filename)
    {
        Ok(h) => Ok(h),
        _ => Err(_ERR_CREATE_FILE),
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
