extern crate zip;

use std::ffi::OsStr;
use std::fs;
use std::fs::File;

use std::io::Read;
use walkdir::WalkDir;
use zip::read::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipError;
use std::ptr;
use winapi::um::fileapi::ReadFile;
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::minwindef::DWORD;

use std::os::windows::io::AsRawHandle;
use windows::Win32::System::Threading::*;
use windows_result::Error;
use std::result::Result;


pub fn open_word() -> Result<(),Error> {
    // let word_x = windows::core::w!("WordDocument");
    // let h_file: HANDLE = ptr::null_mut();
    let file_path = "c:\\Users\\mabia\\projects\\word_processor\\assets\\doc_origem.docx";
    
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut bytes_read: DWORD = 0;
    let overlapped: LPOVERLAPPED = ptr::null_mut();
    let file = File::open(file_path)?;
    let file_handle: *mut winapi::ctypes::c_void = file.as_raw_handle() as *mut winapi::ctypes::c_void;
    let success = unsafe {
        ReadFile(
            file_handle,
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
    Ok(())
}

static COUNTER: std::sync::RwLock<i32> = std::sync::RwLock::new(0);

pub fn open_any_file(file_path: &str) -> Result<(), Error> {
 
    unsafe {
        let work = CreateThreadpoolWork(Some(callback), None, None)?;

        for _ in 0..10 {
            SubmitThreadpoolWork(work);
        }

        WaitForThreadpoolWorkCallbacks(work, false);
    }

    let counter = COUNTER.read().unwrap();
    println!("counter: {}", *counter);
    Ok(())
        
}

extern "system" fn callback(_: PTP_CALLBACK_INSTANCE, _: *mut std::ffi::c_void, _: PTP_WORK) {
    let mut counter = COUNTER.write().unwrap();
    *counter += 1;
}


pub fn show_macros(file_name: &str) -> Vec<String> {

    let mut macro_name = Vec::new();

    match listar_macros(file_name) {
        Ok(macros) => {
            println!("Macros encontradas:");
            for name in macros.iter() {
                println!("{}", name);
                macro_name.push(name.to_string());
            }
        }
        Err(err) => {
            eprintln!("Ero ao lista macros {}", err);
        },
    }

    return macro_name;

}

fn listar_macros(file_name: &str) -> Result<Vec<String>, String> {
    let dir_path ="C:\\Users\\mabia\\projects\\repos";
    let mut macros:Vec<String> = Vec::new();

    for result in WalkDir::new(dir_path) {
        match  result {
            Ok(file_name_int) => {
                println!("file name: {:?}", file_name_int.file_name());
                macros.push(file_name_int.file_name().to_string_lossy().to_string());
                if file_name_int.file_name() == "vbaProject.bin" {
                    let mut vba_project_bin = String::new();
                    // let mut start_index = start_pos + "word/".len();
                }    
            },
            Err(_) => todo!(),
        }
    };


    Ok(macros)
}

fn v1_listar_macros(file_name: &str) -> Result<Vec<String>, String> {
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };                                   


    let mut archive = match ZipArchive::new(file) {
        Ok(file) => file,
        Err(e) => {
            let zip_error_message = match e {
                ZipError::Io(io_err) => io_err.to_string(),
                _ => format!("Erro ao abrir o arquivo ZIP: {}", e),
            };
            let io_error = Error::new(windows_result::HRESULT::default(), zip_error_message);
            //return Err(io_error);
            return Err(io_error.to_string());
        },
    };

    let mut macros = Vec::new();
    let mut contents = String::new();
    println!("zip files count :{}", archive.len());


    for i in 0..archive.len() {
        // let mut file = match archive.by_index(i) {
        //     Ok(file) => file,
        //     Err(err) => {
        //         let zip_error_message = match err {
        //             ZipError::Io(io_err) => io_err.to_string(),
        //             _ => format!("Erro ao abrir o arquifo de dentro do arquivo ZIP: {}", err),
        //         };
        //         let io_error = Error::new(windows_result::HRESULT::default(), zip_error_message);                
        //         return Err(io_error.to_string());
        //     }            
        // };
        let mut file = archive.by_index(i).unwrap();

        println!("arquivo com final xml: {:?}", file.enclosed_name() );            
        // if file.name() == "word/document.xml" {                        
        //     file.read_to_string(&mut contents).map_err(|e| {
        //         return e;
        //     });
        //     // break;
        // }
    }
    // println!("conteudo do arquivo: \n{}", contents);

    if let Some(start_pos) = contents.find("word/vbaProject.bin") {
        let mut vba_project_bin = String::new();
        let mut start_index = start_pos + "word/".len();

        while let Some(end_pos) = contents[start_index..].find("\"") {
            let end_index = start_index + end_pos;
            vba_project_bin.push_str(&contents[start_index..end_index]);
            start_index = end_index + 1;
            if vba_project_bin.ends_with(".bin") {
                break;
            }
        }
    

        for i in 0..archive.len() {
            let mut file = match archive.by_index(i) {
                Ok(file) => file,
                Err(err) => {
                    let zip_error_message = match err {
                        ZipError::Io(io_err) => io_err.to_string(),
                        _ => format!("Erro ao abrir o arquifo de dentro do arquivo ZIP: {}", err),
                    };
                    let io_error = Error::new(windows_result::HRESULT::default(), zip_error_message);                
                    return Err(io_error.to_string());
                }                               
            };

            if file.name() == &vba_project_bin {
                let mut vba_data_xml = String::new();
                file.read_to_string(&mut vba_data_xml).map_err(|e| {
                    return e;   
                });
                let macro_start = "<w:binData";
                let macro_end = "</w:binData>";
                let mut start_index = 0;
                while let Some(start_pos) = contents[start_index..].find(macro_start) {
                    start_index = start_index + start_pos;
                    if let Some(end_pos) = contents[start_index..].find(macro_end) {
                        let end_index = start_index + end_pos + macro_end.len();
                        let macro_content = &contents[start_index..end_index];
                        // extrai nome da macro
                        if let Some(name_start) = macro_content.find("Sub ") {
                            let name_end = macro_content[name_start..].find('(').unwrap_or(macro_content.len());
                            let macro_name = macro_content[name_start + "Sub ".len()..name_start + name_end].trim();
                            macros.push(macro_name.to_string());
                        }
                        start_index = end_index;
                    } else {
                        break;
                    }
                }
            }

        }

        //     let macro_start = "<w:binData";
        //     let macro_end = "</w:binData>";
        //     let mut start_index = 0;
        //     while let Some(start_pos) = contents[start_index..].find(macro_start) {
        //         start_index = start_index + start_pos;
        //         if let Some(end_pos) = contents[start_index..].find(macro_end) {
        //             let end_index = start_index + end_pos + macro_end.len();
        //             let macro_content = &contents[start_index..end_index];
        //             // extrai nome da macro
        //             if let Some(name_start) = macro_content.find("Sub ") {
        //                 let name_end = macro_content[name_start..].find('(').unwrap_or(macro_content.len());
        //                 let macro_name = macro_content[name_start + "Sub ".len()..name_start + name_end].trim();
        //                 macros.push(macro_name.to_string());
        //             }
        //             start_index = end_index;
        //         } else {
        //             break;
        //         }
        //     }
        // } else {
        //     println!("{}", file.name());
        // }
       
    }

    Ok(macros)
}



