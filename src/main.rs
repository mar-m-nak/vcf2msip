mod vcf_parser;
mod ini_io;
mod error_flg;
mod arg_and_help;
mod file_fns;
mod xml_perser;
mod progress_bar;

use std::process::exit;
use vcf_parser::*;
use ini_io::*;
use error_flg::*;
use arg_and_help::*;
use file_fns::*;
use xml_perser::*;
use progress_bar::*;

fn main() {
    // cargo run -- -r .\testfiles\contacts.vcf .\testfiles\Contacts.xml

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

    // check i/o file exists
    if !is_exists_io_files(&args) {
        return Err(_ERR_FILE_NOT_FOUND);
    }

    // get MicroSIP Contacts
    let mut sip_contacts = if args.is_merge() {
        match SipContacts::new(&args.save_file_name()) {
            Ok(sc) => sc,
            Err(e) => { return Err(e); },
        }
    } else {
        SipContacts::empty()
    };

    // open and read vcf file
    let vcf = match Vcf::new(&args.load_file_name()) {
        Ok(vcf) => vcf,
        Err(e) => { return Err(e); },
    };

    // output new micro-sip xml to temporaly file
    let tmp_filename = make_tmp_filename(&args.save_file_name());
    let mut hfile = match create_file(&tmp_filename, false) {
        Ok(h) => h,
        Err(e) => { return Err(e); }
    };
    if let Err(e) = output_xml_file(&vcf, &mut hfile, &mut sip_contacts) {
        delete_file(&tmp_filename);
        return Err(e);
    }
    // backup original micro-sip xml, if necessary
    if !args.is_no_bup() {
        if let Err(e) = file_backup(&args.save_file_name()) {
            delete_file(&tmp_filename);
            return Err(e);
        }
    }
    // apply tempolary file to true micro-sip xml file
    if let Err(_) = rename(&tmp_filename, &args.save_file_name()) {
        delete_file(&tmp_filename);
        return Err(_ERR_FIX_FILE_COPY);
    }

    // renew logs name for MicroSIP.ini
    if args.is_renew_logs() {
        // read ini file to buffer in IniIo
        let mut ini_io = match IniIo::new(&args.microsip_ini_file()) {
            Ok(iniio) => iniio,
            Err(_) => { return Err(_ERR_DID_NOT_RUN_RENEW_LOGS) },
        };
        renew_ini_buffer(&vcf, &mut ini_io);
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

/// write to xml file
fn output_xml_file(vcf: &Vcf, hfile: &mut File, sip_contacts: &mut SipContacts) -> Result<(), i32> {
    // write header
    if let Err(_) = writeln!(hfile, "<?xml version=\"1.0\"?>\r\n<contacts>\r") {
        return Err(_ERR_WRITE_FILE);
    }
    // loop at vcards
    let mut count_contact: usize = 0;
    let mut count_number: usize = 0;
    let mut count_merge: usize = 0;
    let vcf_vcards = vcf.get_vcards();
    let mut pgbar = ProgressBar::new("Convert", vcf_vcards.len());
    for vcard in vcf_vcards {
        pgbar.progress();
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
            // clear original contact if exist
            if !sip_contacts.is_empty() {
                sip_contacts.clear_exist(&number);
            };
            // write element
            let xml = Contact::fmt_xml(&name, tel_type.as_ref(), &number);
            if let Err(_) = writeln!(hfile, "{}\r", xml) {
                continue;
            }
            count_number += 1;
        }
        count_contact += 1;
    }
    // merge remaining original contact
    if !sip_contacts.is_empty() {
        let tel_type = "";
        let mut pgbar = ProgressBar::new("Merge", sip_contacts.data().len());
        for (sct_fix_number, number, name) in sip_contacts.data() {
            pgbar.progress();
            if sct_fix_number.is_empty() { continue; }
            // write element
            let xml = Contact::fmt_xml(&name, tel_type, &number);
            if let Err(_) = writeln!(hfile, "{}\r", xml) {
                continue;
            }
            count_merge += 1;
        }
    }
    // write footer
    if let Err(_) = writeln!(hfile, "</contacts>\r") {
        return Err(_ERR_WRITE_FILE);
    }
    println!("contact: {} / number: {} / merge: {}", count_contact, count_number, count_merge);
    Ok(())
}

/// replace MicroSIP.ini on buffer in IniIo
fn renew_ini_buffer(vcf: &Vcf, ini_io: &mut IniIo) {
    // loop at vcards
    let vcf_vcards = vcf.get_vcards();
    let mut pgbar = ProgressBar::new("ReNew Logs", vcf_vcards.len());
    for vcard in vcf_vcards {
        pgbar.progress();
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
}