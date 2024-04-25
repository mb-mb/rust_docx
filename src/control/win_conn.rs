use std::ptr;
use winapi::um::fileapi::ReadFile;
use winapi::um::winnt::HANDLE;
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::minwindef::DWORD;

pub fn open_word() {
    // let word_x = windows::core::w!("WordDocument");
    let h_file: HANDLE = ptr::null_mut();
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut bytes_read: DWORD = 0;
    let overlapped: LPOVERLAPPED = ptr::null_mut();

    let success = unsafe {
        ReadFile(
            h_file,
            buffer.as_mut_ptr() as LPVOID,
            buffer.len() as DWORD,
            &mut bytes_read,
            overlapped,
        )
    };

    if success != 0 {
        println!("Read {} bytes from file.", bytes_read);
    } else {
        println!("Error reading file.");
    }
}