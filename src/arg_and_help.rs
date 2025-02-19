use crate::file_fns;
use file_fns::*;

pub const ARG_HELP: &'static [&'static str] = &["-h", "-v", "--help", "--version"];
pub const ARG_MERGE: &'static [&'static str] = &["-m", "--merge"];
pub const ARG_OVERWRITE: &'static [&'static str] = &["-n", "--no-bup"];
pub const ARG_RENEWLOGS: &'static [&'static str] = &["-r", "--renew-logs"];
pub const ARG_OUTPUT_CSV_AGEPHONE: &'static [&'static str] = &["-ca", "--output-csv-agephone"];

pub const ARG_PAT_NAME: &'static str = "%name%";
pub const ARG_PAT_FIRST_INITIAL: &'static str = "%finitial%";
pub const ARG_PAT_LAST_INITIAL: &'static str = "%linitial%";
pub const ARG_PAT_TEL_TYPE: &'static str = "%teltype%";
pub const ARG_PAT_CATEGORIES: &'static str = "%categories%";
pub const ARG_PAT_DEFAULT: &'static str = "%linitial% - %name% (%teltype%)";

const _PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const _PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const _PKG_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const _PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Debug, Default)]
pub struct Args {
    load_file_name: String,
    save_file_name: String,
    microsip_ini_file: String,
    name_pattern_normal: String,
    is_help: bool,
    is_merge: bool,
    is_no_bup: bool,
    is_renew_logs: bool,
    is_output_csv_agephone: bool
}

impl Args {

    /// Set structure from console args
    pub fn get_params() -> Self {
        let mut args = Args::default();
        let mut usrtxt_count = 0;
        let ms = MAIN_SEPARATOR.to_string();
        args.name_pattern_normal = ARG_PAT_DEFAULT.to_string();
        for (i, arg) in env::args().enumerate() {
            if i == 0 { continue; }
            if ARG_HELP.contains(&arg.as_ref()) { args.is_help = true; break; }
            else if ARG_MERGE.contains(&arg.as_ref()) { args.is_merge = true; }
            else if ARG_OVERWRITE.contains(&arg.as_ref()) { args.is_no_bup = true; }
            else if ARG_RENEWLOGS.contains(&arg.as_ref()) { args.is_renew_logs = true; }
            else if ARG_OUTPUT_CSV_AGEPHONE.contains(&arg.as_ref()) { args.is_output_csv_agephone = true; }
            else {
                match usrtxt_count {
                    0 => args.load_file_name = arg.replace("/", &ms),
                    1 => args.save_file_name = arg.replace("/", &ms),
                    2 => args.name_pattern_normal = arg,
                    _ => { args.is_help = true; break; },
                }
                usrtxt_count += 1;
            }
        }
        #[cfg(debug_assertions)] {
            // Debug
            // cargo run -- -m -r -n .\sandbox\contacts.vcf .\sandbox\Contacts.xml
            // or ...
            // args.is_help = false;
            // usrtxt_count = 4;
            // args.load_file_name = r".\sandbox\contacts.vcf".to_string();
            // args.save_file_name = r".\sandbox\Contacts.xml".to_string();
            // args.is_merge = true;
            // args.is_no_bup = true;
            // args.is_renew_logs = true;
            // args.is_output_csv_agephone = false;
            // args.name_pattern_normal = r"%linitial% - %name% (%teltype%)".to_string();
        }

        // Out of file and name pattern arg count
        if usrtxt_count < 2 || usrtxt_count > 4 {
            args.is_help = true;
        }
        // Check file name
        if let Some(s) = Path::new(&args.load_file_name).extension() {
            let ext = s.to_str().map_or("", |s| s);
            if ext.to_lowercase().as_str() != "vcf" {
                args.is_help = true;
            }
        }
        if args.is_output_csv_agephone {
            args.is_merge = false;
            args.is_renew_logs = false;
            args.microsip_ini_file = "".to_string();
        } else {
            // Check file name
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
        }
        #[cfg(debug_assertions)] { println!("{:?}", args); }
        args
    }

    pub fn load_file_name(&self) -> &str { self.load_file_name.as_ref() }
    pub fn save_file_name(&self) -> &str { self.save_file_name.as_ref() }
    pub fn microsip_ini_file(&self) -> &str { self.microsip_ini_file.as_ref() }
    pub fn name_pattern_normal(&self) -> &str { self.name_pattern_normal.as_ref() }
    pub fn is_help(&self) -> bool { self.is_help }
    pub fn is_merge(&self) -> bool { self.is_merge }
    pub fn is_no_bup(&self) -> bool { self.is_no_bup }
    pub fn is_renew_logs(&self) -> bool { self.is_renew_logs }
    pub fn is_output_csv_agephone(&self) -> bool { self.is_output_csv_agephone }

    pub fn print_help(&self) {
        println!("\n\n{} - Version {} : by {}", _PKG_NAME, _PKG_VERSION, _PKG_AUTHORS);
        println!("{}", _PKG_DESCRIPTION);
        println!("\nusage: {} [OPTIONS] \
            \"path\\to\\load\\*.vcf\" \
            \"path\\to\\save\\Contacts.xml\" \
            [\"%PATTERN%\"]",
            _PKG_NAME
        );
        println!("\n---- OPTIONS ----");
        println!("{:?}\t... Merge from exist MicroSIP contacts too. Default: no merge.", ARG_MERGE);
        println!("{:?}\t... Do not create backup. Default: create backup.", ARG_OVERWRITE);
        println!("{:?}\t... Renew name in logs tab. Default: no touch.", ARG_RENEWLOGS);
        println!("{:?}\t... Just convert to CSV for AGEphone. Default: no.", ARG_OUTPUT_CSV_AGEPHONE);
        println!("{:?}\t... This message.", ARG_HELP);
        println!("\n---- PATTERN ----");
        println!("- Pattern of convert to name from vcf contact.");
        println!("- Apply to Name column in MicroSIP contacts (and logs if --renew-logs) tab.");
        println!("- Emptied () and [] are remove at all last.");
        println!("- Default: \"{}\"", ARG_PAT_DEFAULT);
        println!("{:?}\t... Full name or Organization name.", ARG_PAT_NAME);
        println!("{:?}\t... Initial of first name or %name%", ARG_PAT_FIRST_INITIAL);
        println!("{:?}\t... Initial of last name or %name%", ARG_PAT_LAST_INITIAL);
        println!("{:?}\t... Telephone type.", ARG_PAT_TEL_TYPE);
        println!("{:?}\t... Categories string.", ARG_PAT_CATEGORIES);
        println!("\n");
    }
}
