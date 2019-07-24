#[cfg(windows)]
extern crate winapi;

use std::ffi::CString;
// extern crate schedule_recv;

use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
use winapi::um::winbase::{ReadDirectoryChangesW, FILE_FLAG_BACKUP_SEMANTICS};

use winapi::um::winnt::{FILE_LIST_DIRECTORY, FILE_NOTIFY_CHANGE_FILE_NAME, FILE_SHARE_READ};

fn main() {
    unsafe {
        // ..
        let hDir = CreateFileA(
            CString::new("C:/Users/Admin/Desktop/tmp/")
                .unwrap()
                .as_ptr(),
            FILE_LIST_DIRECTORY,
            FILE_SHARE_READ,
            None.unwrap(),
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            None.unwrap(),
        );
        let x = ReadDirectoryChangesW(
            hDir,
            std::ptr::null_mut(),
            0,
            0,
            FILE_NOTIFY_CHANGE_FILE_NAME,
            std::ptr::null_mut(),
            None.unwrap(),
            None.unwrap(),
        );
        // ...
    }
}