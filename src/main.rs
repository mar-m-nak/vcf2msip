mod vcf_parser;
mod ini_io;
mod error_flg;
mod arg_and_help;
mod file_fns;

use std::process::exit;
use vcf_parser::*;
use ini_io::*;
use error_flg::*;
use arg_and_help::*;
use file_fns::*;

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
    let mut hxmlfile = match create_file(&filename, false) {
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
                let old_line = ini_io.get_match_number_line(tel.get_number());
                // replace MicroSIP.ini on buffer
                if !old_line.is_empty() {
                    let new_name = format!("{} ({})", name, tel.get_type());
                    let new_line = IniIo::make_new_number_line(&old_line, &new_name);
                    if !new_line.is_empty() {
                        ini_io.replace(&old_line, &new_line);
                    }
                }
            }
        }
        // write renewed MicroSIP.ini to tempolary file
        let tmp_filename = make_tmp_filename(&args.microsip_ini_file());
        if let Err(e) = ini_io.save(&tmp_filename) {
            delete_file(&tmp_filename);
            return Err(e);
        }
        // backup MicroSIP.ini, if necessary
        if !args.is_no_bup() {
            if let Err(e) = file_backup(&args.microsip_ini_file()) {
                delete_file(&tmp_filename);
                return Err(e);
            }
        }
        // apply tempolary file to true MicroSIP.ini file
        match rename(&tmp_filename, &args.microsip_ini_file()) {
            Ok(()) => (),
            Err(_) => { return Err(_ERR_FIX_FILE_COPY); },
        }
    }

    Ok(())
}
