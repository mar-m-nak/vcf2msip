use std::{env, path::Path};

pub const ARG_HELP: &'static [&'static str] = &["-h", "-v", "--help", "--version"];
pub const ARG_MERGE: &'static [&'static str] = &["-m", "--merge"];
pub const ARG_OVERWRITE: &'static [&'static str] = &["-n", "--no-bup"];
pub const ARG_RENEWLOGS: &'static [&'static str] = &["-r", "--renew-logs"];

const _PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const _PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

#[derive(Debug, Default)]
pub struct Args {
    load_file_name: String,
    save_file_name: String,
    microsip_ini_file: String,
    is_help: bool,
    is_merge: bool,
    is_no_bup: bool,
    is_renew_logs: bool,
}

impl Args {

    pub fn get_params() -> Self {
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

    pub fn load_file_name(&self) -> &str { self.load_file_name.as_ref() }
    pub fn save_file_name(&self) -> &str { self.save_file_name.as_ref() }
    pub fn microsip_ini_file(&self) -> &str { self.microsip_ini_file.as_ref() }

    pub fn is_help(&self) -> bool { self.is_help }
    pub fn is_merge(&self) -> bool { self.is_merge }
    pub fn is_no_bup(&self) -> bool { self.is_no_bup }
    pub fn is_renew_logs(&self) -> bool { self.is_renew_logs }

    pub fn print_help(&self) {
        println!("\n\n{} - Version {}\n----", _PKG_NAME, _PKG_VERSION);
        println!("usage: {} [OPTIONS] \
            \"path\\to\\load\\*.vcf\" \
            \"path\\to\\save\\Contacts.xml\"",
            _PKG_NAME
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
    }

}
