use crate::file_fns;
use file_fns::*;

pub const ARG_HELP: &'static [&'static str] = &["-h", "-v", "--help", "--version"];
pub const ARG_MERGE: &'static [&'static str] = &["-m", "--merge"];
pub const ARG_OVERWRITE: &'static [&'static str] = &["-n", "--no-bup"];
pub const ARG_RENEWLOGS: &'static [&'static str] = &["-r", "--renew-logs"];

pub const ARG_PAT_NAME: &'static str = "%name%";
pub const ARG_PAT_INITIAL: &'static str = "%initial%";
pub const ARG_PAT_TEL_TYPE: &'static str = "%tel_type%";
pub const ARG_PAT_CATEGORIES: &'static str = "%categories%";
pub const ARG_PAT_DEFAULT: &'static str = "%initial% - %name% (%tel_type%)";
pub const ARG_PAT_LOGS_DEFAULT: &'static str = "%name% (%tel_type%)";

const _PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const _PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

#[derive(Debug, Default)]
pub struct Args {
    load_file_name: String,
    save_file_name: String,
    microsip_ini_file: String,
    name_pattern_normal: String,
    name_pattern_logs: String,
    is_help: bool,
    is_merge: bool,
    is_no_bup: bool,
    is_renew_logs: bool,
}

impl Args {

    /// set structue from console args
    pub fn get_params() -> Self {
        let mut args = Args::default();
        let mut file_count = 0;
        args.name_pattern_normal = ARG_PAT_DEFAULT.to_string();
        args.name_pattern_logs = ARG_PAT_LOGS_DEFAULT.to_string();
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
                let ms = MAIN_SEPARATOR.to_string();
                if file_count == 0 {
                    args.load_file_name = arg.replace("/", &ms);
                } else if file_count == 1 {
                    args.save_file_name = arg.replace("/", &ms);
                } else if file_count == 2 {
                    args.name_pattern_normal = arg;
                } else if file_count == 3 {
                    args.name_pattern_logs = arg;
                } else {
                    args.is_help = true;
                }
                file_count += 1;
            }
        }
        // default name patttern
        if file_count == 3 {
            args.name_pattern_logs = ARG_PAT_LOGS_DEFAULT.to_string();
        }
        // file arg miss match are help
        if file_count < 2 || file_count > 4 {
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
                    args.microsip_ini_file = format!("{}{}MicroSIP.ini", path, MAIN_SEPARATOR);
                }
            }
        }
        #[cfg(debug_assertions)] { println!("{:?}", args); }
        args
    }

    // getters
    pub fn load_file_name(&self) -> &str { self.load_file_name.as_ref() }
    pub fn save_file_name(&self) -> &str { self.save_file_name.as_ref() }
    pub fn microsip_ini_file(&self) -> &str { self.microsip_ini_file.as_ref() }
    pub fn name_pattern_normal(&self) -> &str { self.name_pattern_normal.as_ref() }
    pub fn name_pattern_logs(&self) -> &str { self.name_pattern_logs.as_ref() }
    pub fn is_help(&self) -> bool { self.is_help }
    pub fn is_merge(&self) -> bool { self.is_merge }
    pub fn is_no_bup(&self) -> bool { self.is_no_bup }
    pub fn is_renew_logs(&self) -> bool { self.is_renew_logs }

    /// print help to console
    pub fn print_help(&self) {
        println!("\n\n{} - Version {}", _PKG_NAME, _PKG_VERSION);
        println!("\nusage: {} [OPTIONS] \
            \"path\\to\\load\\*.vcf\" \
            \"path\\to\\save\\Contacts.xml\" \
            [\"%PATTERN1%\"] \
            [\"%PATTERN2%\"]",
            _PKG_NAME
        );
        println!("\n---- OPTIONS ----");
        println!("{:?}\t... Merge from exist MicroSIP contacts too. Default:no merge.", ARG_MERGE);
        println!("{:?}\t... Not create backup. Default:create.", ARG_OVERWRITE);
        println!("{:?}\t... Renew name in logs tab. Default:no touch.", ARG_RENEWLOGS);
        println!("{:?} ... This message.", ARG_HELP);
        println!("\n---- PATTERN1&2 ----");
        println!("- Pattern of convert to name from vcf contact.");
        println!("- PATTERN1 apply to Name column in MicroSIP contacts tab.");
        println!("- PATTERN2 apply to Name column in MicroSIP logs tab.");
        println!("- If omit PATTERN2 then applies PATTERN1 to PATTERN2.");
        println!("- Emptied () and [] are remove at all last.");
        println!("- Default 1: \"{}\"", ARG_PAT_DEFAULT);
        println!("- Default 2: \"{}\"", ARG_PAT_LOGS_DEFAULT);
        println!("{:?}\t... Full name or Organization name.", ARG_PAT_NAME);
        println!("{:?}\t... Initial of %name%", ARG_PAT_INITIAL);
        println!("{:?}\t... Telephone type.", ARG_PAT_TEL_TYPE);
        println!("{:?}\t... Categories string.", ARG_PAT_CATEGORIES);
        println!("\n");
    }
}
