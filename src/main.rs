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

#[derive(Default)]
struct ProcCounter {
    all_contact: usize,
    all_telephone: usize,
    contact: usize,
    telphone: usize,
    merge: usize,
    logs: usize,
}

impl ProcCounter {
    pub fn add_count(&mut self, pc: &ProcCounter) {
        self.all_contact += pc.all_contact;
        self.all_telephone += pc.all_telephone;
        self.contact += pc.contact;
        self.telphone += pc.telphone;
        self.merge += pc.merge;
        self.logs += pc.logs;
    }
    pub fn print(&self) {
        println!(
            "ALL VCF CONTACTS: {} / ALL VCF TELEPHONES: {}",
            self.all_contact, self.all_telephone
        );
        println!(
            "PROCESSED [Contact:{}, Telephone:{}, Merge:{}, RenewLogs:{}]",
            self.contact,
            self.telphone,
            self.merge,
            self.logs
        );
    }
}

fn main() {
    // TODO: test
    // cargo run -- -r .\sandbox\contacts.vcf .\sandbox\Contacts.xml

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

/// Proccess of convert
fn conv(args: &Args) -> Result<(), i32> {

    if !is_exists_io_files(&args) {
        return Err(_ERR_FILE_NOT_FOUND);
    }

    // Read MicroSIP Contacts.xml file
    let mut sip_contacts = if args.is_merge() {
        match SipContacts::new(&args.save_file_name()) {
            Ok(sc) => sc,
            Err(e) => { return Err(e); },
        }
    } else {
        SipContacts::empty()
    };

    // Read vcf file
    let vcf = match Vcf::new(&args.load_file_name()) {
        Ok(vcf) => vcf,
        Err(e) => { return Err(e); },
    };

    // Output new xml to temporaly file
    let tmp_filename = make_tmp_filename(&args.save_file_name());
    let mut hfile = match File::create(&tmp_filename) {
        Ok(h) => h,
        Err(_) => { return Err(_ERR_CREATE_FILE); }
    };
    let mut pc = ProcCounter::default();
    match output_xml_file(&vcf, &args,&mut hfile, &mut sip_contacts) {
        Ok(res_pc) => {
            pc.add_count(&res_pc);
        },
        Err(e) => {
            delete_file(&tmp_filename);
            return Err(e);
        },
    };
    // Backup original xml file
    if !args.is_no_bup() {
        if let Err(e) = file_backup(&args.save_file_name()) {
            delete_file(&tmp_filename);
            return Err(e);
        }
    }
    // Apply tempolary file to true Contact.xml file
    if let Err(_) = rename(&tmp_filename, &args.save_file_name()) {
        delete_file(&tmp_filename);
        return Err(_ERR_FIX_FILE_COPY);
    }

    // Renew logs name in MicroSIP.ini
    if args.is_renew_logs() {
        // Read ini file to buffer
        let mut ini_io = match IniIo::new(&args.microsip_ini_file()) {
            Ok(iniio) => iniio,
            Err(_) => { return Err(_ERR_DID_NOT_RUN_RENEW_LOGS) },
        };
        // Output renewed buffer to tempolary file
        pc.add_count(&renew_ini_buffer(&vcf, &args, &mut ini_io));
        let tmp_filename = make_tmp_filename(&args.microsip_ini_file());
        if let Err(e) = ini_io.save(&tmp_filename) {
            delete_file(&tmp_filename);
            return Err(e);
        }
        // Backup original ini file
        if !args.is_no_bup() {
            if let Err(e) = file_backup(&args.microsip_ini_file()) {
                delete_file(&tmp_filename);
                return Err(e);
            }
        }
        // Apply tempolary file to true MicroSIP.ini file
        match rename(&tmp_filename, &args.microsip_ini_file()) {
            Ok(()) => (),
            Err(_) => { return Err(_ERR_FIX_FILE_COPY); },
        }
    }
    pc.print();
    Ok(())
}

/// Write to xml file
fn output_xml_file(
    vcf: &Vcf, args: &Args, hfile: &mut File, sip_contacts: &mut SipContacts
) -> Result<ProcCounter, i32> {
    // Write start tag
    if let Err(_) = writeln!(hfile, "<?xml version=\"1.0\"?>\r\n<contacts>\r") {
        return Err(_ERR_WRITE_FILE);
    }
    // Loop at vcards
    let mut pc = ProcCounter::default();
    let vcf_vcards = vcf.get_vcards();
    pc.all_contact = vcf_vcards.len();
    let mut pgbar = ProgressBar::new("Convert", pc.all_contact);
    for vcard in vcf_vcards {
        pgbar.progress();
        // Parse one contact, Loop at telephone
        let ct = Contact::new(&vcard);
        if ct.is_empty() { continue; }
        let finitial = ct.finitial();
        let linitial = ct.linitial();
        for tel in ct.tel_iter() {
            pc.all_telephone += 1;
            let number = tel.number();
            // Clear original contact for merge
            if !sip_contacts.is_empty() {
                sip_contacts.clear_exist(number);
            };
            // Write one element
            let new_name = ct.fmt_name(
                &args.name_pattern_normal(), &finitial, &linitial, tel.teltype()
            ).replace("\"", "&quot;");
            if let Err(_) = writeln!(hfile, "{}\r", Contact::xml_line(&new_name, number)) {
                continue;
            }
            pc.telphone += 1;
        }
        pc.contact += 1;
    }
    // Merge remaining original contact
    if !sip_contacts.is_empty() {
        let mut pgbar = ProgressBar::new("Merge", sip_contacts.data().len());
        for (sct_fix_number, number, name) in sip_contacts.data() {
            pgbar.progress();
            if sct_fix_number.is_empty() { continue; }
            // Write one element
            if let Err(_) = writeln!(hfile, "{}\r", Contact::xml_line(&name, &number)) {
                continue;
            }
            pc.merge += 1;
        }
    }
    // Write end tag
    if let Err(_) = writeln!(hfile, "</contacts>\r") {
        return Err(_ERR_WRITE_FILE);
    }
    Ok(pc)
}

/// Replace MicroSIP.ini on buffer
fn renew_ini_buffer(vcf: &Vcf, args: &Args, ini_io: &mut IniIo) -> ProcCounter {
    let mut pc = ProcCounter::default();
    let vcf_vcards = vcf.get_vcards();
    let mut pgbar = ProgressBar::new("ReNew Logs", vcf_vcards.len());
    for vcard in vcf_vcards {
        // Parse one contact, Loop at telephone
        pgbar.progress();
        let ct = Contact::new(&vcard);
        if ct.is_empty() { continue; }
        let finitial = ct.finitial();
        let linitial = ct.linitial();
        for tel in ct.tel_iter() {
            // Replace buffer
            let old_line = ini_io.get_match_number_line(tel.number());
            if !old_line.is_empty() {
                let new_name = ct.fmt_name(
                    &args.name_pattern_logs(), &finitial, &linitial, tel.teltype()
                ).replace(";", "|");
                let new_line = IniIo::make_new_number_line(&old_line, &new_name);
                if !new_line.is_empty() {
                    ini_io.replace(&old_line, &new_line);
                    pc.logs += 1;
                }
            }
        }
    }
    pc
}
