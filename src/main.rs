mod vcf_parser;
mod ini_io;

use std::{cmp::Ordering, env, fs::{OpenOptions, remove_file}, path::Path};
use std::io::prelude::*;
use std::process::exit;
use vcf_parser::*;
use ini_io::{IniIo, _ERR_DID_NOT_RUN_RENEW_LOGS};

#[derive(Debug, Default)]
struct Args {
    load_file_name: String,
    save_file_name: String,
    microsip_ini_file: String,
    is_help: bool,
    is_merge: bool,
    is_no_bup: bool,
    is_renew_logs: bool,
}

const ARG_HELP: &'static [&'static str] = &["-h", "-v", "--help", "--version"];
const ARG_MERGE: &'static [&'static str] = &["-m", "--merge"];
const ARG_OVERWRITE: &'static [&'static str] = &["-n", "--no-bup"];
const ARG_RENEWLOGS: &'static [&'static str] = &["-r", "--renew-logs"];

impl Args {
    fn get_params() -> Self {
        let mut args = Args::default();
        let mut file_count = 0;
        for (i, arg) in env::args().enumerate() {
            if i == 0 { continue; }
            if ARG_HELP.contains(&arg.as_ref()) {
                args.is_help = true;
            } else if ARG_MERGE.contains(&arg.as_ref()) {
                args.is_merge = true;
            } else if ARG_OVERWRITE.contains(&arg.as_ref()) {
                args.is_no_bup = true;
            } else if ARG_RENEWLOGS.contains(&arg.as_ref()) {
                args.is_renew_logs = true;
            } else {
                if file_count == 0 {
                    args.load_file_name = arg;
                } else if file_count == 1 {
                    args.save_file_name = arg;
                } else {
                    args.is_help = true;
                }
                file_count += 1;
            }
        }
        // file arg miss match are help
        if file_count != 2 {
            args.is_help = true;
        }
        if let Some(s) = Path::new(&args.load_file_name).extension() {
            let ext = s.to_str().map_or("", |s| s);
            if ext.to_lowercase().as_str() != "vcf" {
                args.is_help = true;
            }
        }
        if let Some(s) = Path::new(&args.save_file_name).file_name() {
            let fname = s.to_str().map_or("", |s| s);
            if fname.to_lowercase().as_str() != "contacts.xml" {
                args.is_help = true;
            } else {
                // make MicroSIP.ini path to same as xml path
                if let Some(s) = Path::new(&args.save_file_name).parent() {
                    let path = s.to_str().map_or("", |s| s);
                    args.microsip_ini_file = format!("{}/MicroSIP.ini", path);
                }
            }
        }
        println!("{:?}", args);
        args
    }
}

fn main() {
    const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
    let args = Args::get_params();
    if args.is_help {
        println!("\n\n{} - Version {}\n----", PKG_NAME, PKG_VERSION);
        println!("usage: {} [OPTIONS] \
            \"path\\to\\load\\*.vcf\" \
            \"path\\to\\save\\Contacts.xml\"",
            PKG_NAME
        );
        println!("");
        print!("{:?}", ARG_MERGE);
        println!("\t... Merge to exist xml from vcf.");
        print!("{:?}", ARG_OVERWRITE);
        println!("\t... Overwrite xml. (not create backup)");
        print!("{:?}", ARG_RENEWLOGS);
        println!("\t... Renew name in logs tab.");
        print!("{:?}", ARG_HELP);
        println!("\t... This message.");
        println!("\n");
        return ();
    }

    if let Err(e) = conv(&args) {
        print_err_msg(e);
        exit(e);
    };
}
// TODO: args.is_renew_logs = true で ini の履歴の名前を書き換えたい
fn conv(args: &Args) -> Result<(), i32> {

    // open and read vcf file
    // let filename = "./testfiles/contacts.vcf";
    let filename = args.load_file_name.as_ref();
    let vcf = match Vcf::new(&filename) {
        Ok(vcf) => vcf,
        Err(e) => { return Err(e); },
    };

    // create micro-sip xml file
    // let filename = "./testfiles/Contacts.xml";
    let filename = args.save_file_name.as_ref();
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
    if args.is_renew_logs {
        let mut ini_io = match IniIo::new(&args.microsip_ini_file) {
            Ok(iniio) => iniio,
            Err(_) => { return Err(_ERR_DID_NOT_RUN_RENEW_LOGS) },
        };
        // println!("{:?}", ini_io);

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
                if !old_line.is_empty() {
                    let new_name = format!("{}{}", name, tel_type);
                    let new_line = IniIo::make_new_number_line(&old_line, &new_name);
                    if !new_line.is_empty() {
                        ini_io.replace(&old_line, &new_line);
                    }
                }
            }
        }
        // println!("{:?}", ini_io);
        if let Err(e) = ini_io.save() {
            return Err(e);
        }
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
        _ERR_DID_NOT_RUN_RENEW_LOGS => "MicroSIP.ini は更新されませんでした",
        _ => "",
    };
    if !msg.is_empty() { println!("{}", msg) } else { println!("不明なエラーです {}", e) };
}
