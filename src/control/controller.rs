
use std::{collections::HashMap, ffi::OsString, fs::{self}, io::{Error, Read}};
use docx_rs::*;
use serde_json::Value;
use anyhow;

#[path = "win_conn.rs"] mod win_conn;


#[derive(Debug)]
pub enum LoadResult {
    Success(Value),
    Error(serde_json::Error),
}


#[derive(Default)]
pub struct Controller {
    checkboxes: HashMap<String, bool>,    
    description_file_to_process: String,
}

impl Controller {

    pub fn new(checkboxes: HashMap<String, bool>) -> Self {
        Self {
            checkboxes,
            description_file_to_process: String::from("<escolha o arquivo para processar>"),
        }
    }

    pub fn add_macro(&mut self, id: usize, macro_text: &str) {

        for (box_text, check) in self.checkboxes.iter() {
            println!("{}", box_text);
        };


        self.checkboxes.insert(macro_text.to_string(), false);
        //self.messages.push(format!("Macro {} adicionada!", macro_text));
    }

    fn rem_macro(id: usize, controller: &mut Controller) {
        //controller.messages.push(format!("Macro {} removida!", id.to_string()));
    }    

    pub fn macros(&self) -> HashMap<String, bool> {
        self.checkboxes.clone()
    }
    
    pub fn unprocessed(&self) -> HashMap<String, bool> {
        let mut unproc:HashMap<String, bool> = HashMap::new();
        
        for (item, value) in self.checkboxes.iter() {
            if *value {
                unproc.insert(String::from(item), *value);
            }
        }
        unproc
    }

    pub fn get_macros(&self) -> Vec<(bool, String)> {

        let mut ret_macros : Vec<(bool, String)> = Vec::new();
    
        for (check_name, value) in self.checkboxes.iter() {
            ret_macros.push((*value, check_name.to_string()));
        } 
        ret_macros.sort_by(|a: &(_, String),b| a.cmp(b));
        ret_macros    
    }
    
    pub fn select_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            println!("the user chose : {:?}", Some(path.display().to_string()));
            let filenamex: &std::ffi::OsStr = path.file_name().unwrap();
            let zsx = OsString::from(filenamex);
            if let Ok(regular_string) = zsx.into_string() {
                self.description_file_to_process = regular_string;
            }
        };        
    }

    pub fn file_to_process(&self) -> String {
        self.description_file_to_process.clone()
    }


    pub fn checked_macro(&mut self, is_checked: &mut bool, macro_text: &String) {
        // if let entry = self.checkboxes.entry(macro_text.clone()) {
        //     *entry.or_insert(false) = *is_checked;
        // }

        for (item, checked) in self.checkboxes.iter_mut() {
            if item == macro_text {
                *checked = *is_checked;
                println!("is {} checked {}", is_checked, item);
            }
        }
    }

    pub fn execute_vba_macro(&self, path: String, macro_name: String) -> Result<(), DocxError> {
        //let mut buffer = Vec::new();
        let path = std::path::Path::new(&self.description_file_to_process);    
        let file = std::fs::File::create(path).unwrap();
        let docx = Docx::new()
                            .build()
                            .pack(file)?;
        
        Ok(())

        /*
            let path = std::path::Path::new("./output/examples/hello.docx");
    let file = std::fs::File::create(path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .build()
        .pack(file)?;
    Ok(())
        
         */
                
        // let doc = docx_rs::read_docx(&mut buffer);      
        // if let Ok(doc) = docx_rs::read_docx(&mut buffer) {
            
        //     let macro_code = extract_macro_code(&doc, macro_name);
        //     execute_vba_macro_code(&macro_code);

        //     let buffer = ldoc;
        //     ldoc.write_file(self.description_file_to_process);

        // }      

    }

    fn read_to_vec(self, file_name:&str) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        std::fs::File::open(file_name)?.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn load_word_file(&mut self) -> Result<(), Error> {
        // let hwdnd = crate::control::win_conn::hwnd;
        // let xml_var = crate::control::win_conn::xml_var;
        // let xml_file = crate::control::win_conn::update_bibliography(hwdnd, xml_var);
        let file_path = "c:\\Users\\mabia\\projects\\word_processor\\assets\\doc_origem.docx";
        // crate::control::win_conn::open_word();
        // crate::control::win_conn::open_any_file(file_path);
        let mut macros = crate::control::win_conn::show_macros(&file_path);
        macros.sort();

        for macrox in macros.iter() {
            self.checkboxes.insert(macrox.to_string(), true);
        }
        Ok(())
    
    }

}

pub fn init_checkboxes(ini_file: Value)-> HashMap<String, bool>{
    let mut cb = HashMap::new();

    if let Value::Array(macros) = ini_file {    
        for item in macros.iter() {
            if let Some(macrow) = item.as_str() {
                cb.insert(String::from(macrow),false);    
            }        
        }
    }

    // cb.insert(String::from("TRANSITO"),false);
    // cb.insert(String::from("MAIUSCULAS"), false); 
    // cb.insert(String::from("MINUSCULAS"),false);
    cb
}

pub fn load_ini_file() -> Result<LoadResult,  Error> {
    let file_path = std::path::Path::new("resources/init.cfg"); 
    let file_content = fs::read_to_string(file_path);

    match file_content {
        Ok(file) => {
            let json_file:Value = serde_json::from_str(&file)?;
            return Ok(LoadResult::Success(json_file));
        }
        Err(_err) => Err(_err),
    }

}




//let file_path = std::path::Path::new("resources/init.cfg");
//let file_content = fs::read_to_string(file_path)?; // Use try? for error handling

//let json_file = serde_json::from_str(&file_content)?;

// Access the "macro" item
// let macro_value = json_file.get("macro").and_then(|v| v.as_str());

// match macro_value {
//     Some(value) => Ok(value.to_string()), // Convert string slice to owned String
//     None => Err(serde_json::Error::is_syntax),// Handle missing field
// }
// Ok(())