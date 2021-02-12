mod vcf_parser;
mod ini_io;
mod error_flg;
mod arg_and_help;

use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use std::process::exit;
use vcf_parser::*;
use ini_io::*;
use error_flg::*;
use arg_and_help::*;


fn main() {
    let args = Args::get_params();
    if args.is_help() {
        args.print_help();
        return ();
    }

    if let Err(e) = conv(&args) {
        print_err_msg(e);
        exit(e);
    };
}

/// proccess of convert
fn conv(args: &Args) -> Result<(), i32> {

    // open and read vcf file
    // let filename = "./testfiles/contacts.vcf";
    let filename = args.load_file_name();
    let vcf = match Vcf::new(&filename) {
        Ok(vcf) => vcf,
        Err(e) => { return Err(e); },
    };

    // create micro-sip xml file
    // let filename = "./testfiles/Contacts.xml";
    let filename = args.save_file_name();
    let mut hxmlfile = match create_xml_file(&filename, false) {
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

    // renew logs name for MicroSIP.ini
    if args.is_renew_logs() {
        let mut ini_io = match IniIo::new(&args.microsip_ini_file()) {
            Ok(iniio) => iniio,
            Err(_) => { return Err(_ERR_DID_NOT_RUN_RENEW_LOGS) },
        };
        // loop at vcards
        for vcard in vcf.get_vcards() {
            // parse one contact
            let ct = Contact::new(&vcard);
            if ct.is_empty() { continue; }
            let name = ct.full_name().replace("\"", "");
            // loop at telephone in this contact
            for tel in ct.tel_iter() {
                let number = tel.get_number();
                let tel_type = if tel.get_type().is_empty() {
                    "".to_string()
                } else {
                    format!(" ({})", tel.get_type())
                };
                let old_line = ini_io.get_match_number_line(number);
                // replace MicroSIP.ini on buffer
                if !old_line.is_empty() {
                    let new_name = format!("{}{}", name, tel_type);
                    let new_line = IniIo::make_new_number_line(&old_line, &new_name);
                    if !new_line.is_empty() {
                        ini_io.replace(&old_line, &new_line);
                    }
                }
            }
        }
        // write renewed MicroSIP.ini
        // TODO: 保存は .tmp にしておく
        let filename = "./testfiles/MicroSIP_new.ini";
        if let Err(e) = ini_io.save(&filename) {
            return Err(e);
        }
        // TODO: 保存成功したら .tmp を .ini に上書き
        // TODO: または、.ini をバックアップして .tmp を .ini にリネーム
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
