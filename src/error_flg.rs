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
