// my error flags

pub const _ERR_FILE_NOT_FOUND: i32 = 1;
pub const _ERR_CREATE_FILE: i32 = 2;
pub const _ERR_WRITE_FILE: i32 = 3;
pub const _ERR_READ_FILE: i32 = 4;

pub const _ERR_DID_NOT_RUN_RENEW_LOGS: i32 = 101;
pub const _ERR_WRITE_INI_FILE: i32 = 102;

pub const _ERR_FIX_FILE_COPY: i32 = 201;
pub const _ERR_FILE_BACKUP: i32 = 202;

/// print my error message
pub fn print_err_msg(e: i32) {
    let msg = match e {
        _ERR_FILE_NOT_FOUND => "ファイルが見つかりません",
        _ERR_CREATE_FILE => "ファイル作成に失敗しました",
        _ERR_READ_FILE => "ファイル読み込みに失敗しました",
        _ERR_WRITE_FILE => "ファイル書き込みに失敗しました",
        _ERR_DID_NOT_RUN_RENEW_LOGS => "MicroSIP.ini は更新されませんでした",
        _ERR_WRITE_INI_FILE => "MicroSIP.ini の書き込みに失敗しました",
        _ERR_FIX_FILE_COPY => "作成したファイルの置き換えに失敗しました",
        _ERR_FILE_BACKUP => "ファイルのバックアップに失敗しました",
        _ => "",
    };
    if !msg.is_empty() { println!("{}", msg) } else { println!("不明なエラーです {}", e) };
}

/// return numeric only number
pub fn fix_number(number: &str) -> String {
    let mut fix_number = String::with_capacity(16);
    for c in number.chars() {
        if c.is_numeric() {
            fix_number = format!("{}{}", fix_number, c);
        }
    };
    fix_number
}

/// progress bar
pub fn console_progress_bar(title: &str, all_len: usize, prog_len: usize) {
    const _MAX: usize = 80;
    let width = _MAX - title.len() - 7 - 12;
    let is_done = all_len <= prog_len;
    let per = if !is_done {
        (prog_len as f32 / all_len as f32 * width as f32).ceil() as usize
    } else {
        width
    };
    print!("\r \r> {} : [", &title);
    for i in 1..width {
        let c = if i < per {"#"} else {" "};
        print!("{}", c);
    }
    if !is_done {
        print!("] {} / {}", prog_len, all_len);
    } else {
        let w = prog_len.to_string().len() + all_len.to_string().len() + 4;
        print!("] Done!{:1$}\n", " ", w - 6);
    }
}
