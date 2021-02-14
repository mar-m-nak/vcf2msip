pub const _ERR_FILE_NOT_FOUND: i32 = 1;
pub const _ERR_CREATE_FILE: i32 = 2;
pub const _ERR_WRITE_FILE: i32 = 3;
pub const _ERR_READ_FILE: i32 = 4;

pub const _ERR_DID_NOT_RUN_RENEW_LOGS: i32 = 101;
pub const _ERR_WRITE_INI_FILE: i32 = 102;

pub const _ERR_FIX_FILE_COPY: i32 = 201;
pub const _ERR_FILE_BACKUP: i32 = 202;

pub fn print_err_msg(e: i32) {
    let msg = match e {
        _ERR_FILE_NOT_FOUND => "File not found.",
        _ERR_CREATE_FILE => "File create failed.",
        _ERR_READ_FILE => "File read failed.",
        _ERR_WRITE_FILE => "File write failed.",
        _ERR_DID_NOT_RUN_RENEW_LOGS => "MicroSIP.ini is not renewed.",
        _ERR_WRITE_INI_FILE => "MicroSIP.ini write failed.",
        _ERR_FIX_FILE_COPY => "Failed to replace the created file.",
        _ERR_FILE_BACKUP => "File backup failed.",
        _ => "",
    };
    if !msg.is_empty() { println!("ERROR: {}", msg) } else { println!("Unknown error {}", e) };
}

/// Return numeric only
pub fn fix_number(number: &str) -> String {
    let mut fix_number = String::with_capacity(16);
    for c in number.chars() {
        if c.is_numeric() {
            fix_number = format!("{}{}", fix_number, c);
        }
    };
    fix_number
}
