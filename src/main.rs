#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[path = "control/controller.rs"] mod control;

use std::collections::HashMap;

use control::{init_checkboxes, load_ini_file,  Controller, LoadResult};
use egui::{menu, CentralPanel, Context, Grid, Label, Pos2, SidePanel, TopBottomPanel, Ui, Vec2, ViewportBuilder, ViewportCommand, Visuals};
use eframe::egui;


#[derive(Default)]
struct MyApp {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
    controller: control::Controller,
    pending_updates: bool,
}
 
impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, controller: control::Controller) -> Self {
        _cc.egui_ctx.set_visuals(Visuals::dark());
        // Default::default()        
        Self {
            show_confirmation_dialog: false,
            allowed_to_close: false,
            controller,
            pending_updates: true,
        }
    }
}


impl eframe::App for MyApp {
    
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        if self.pending_updates {
        TopBottomPanel::top("top_panel0").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Programas", |ui| {
                if ui.button("Jpg convert").clicked() {
                    println!("Jpg clicked");                
                }
                if ui.button("Pdf convert").clicked() {
                    println!("Pdf clicked");                
                }
                if ui.button("Livro").clicked() {
                    println!(" clicked");                
                }
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }            
            });
        })});

        SidePanel::left("my_left_panel")
            .min_width(300.0)
            .show(ctx, |ui|{
                ui.heading("Macros");        

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.indent("todo_macros", |ui| {
                        let macros = self.controller.macros();
                        for (item, is_checked) in macros.iter() {
                            ui.add_space(5.0);
                            let mut tessa = is_checked.clone();
                            Grid::new(item.clone()).show(ui, |ui| {
                                // let &mut checked = is_checked; 
                                if ui.checkbox(&mut tessa, item.clone()).clicked() {                                
                                    print!("{}\n", item.to_string());
                                    self.controller.checked_macro(&mut tessa, &item.to_string());
                                }
                            });
                        };     
                                          
                    });
                });
            });


        CentralPanel::default().show(ctx, |_ui| {            
            TopBottomPanel::top("my_central_top_panel").show(ctx, |ui|{
                
                if ui.label("file").clicked() {
                    self.controller.select_file(); 
                    let _ = self.controller.load_word_file();
                    self.pending_updates = false;                    
                }
                ui.heading(self.controller.file_to_process());
            });

            
            SidePanel::left("my_central_left_panel")
                .min_width(251.0)
                .show(ctx, |ui| {
                    ui.heading("a processar");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let unprocessed = self.controller.unprocessed();
                        for (item, _) in unprocessed {        
                            ui.add(Label::new(item));
                        }
                       //self.pending_updates = false;
                    });
                });

            SidePanel::right("my_central_right_panel")
                    .min_width(251.0)
                    .show(ctx, |ui|{
                        ui.heading("processadas");
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                if self.allowed_to_close {
                    // do nothing
                } else {
                    ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                    self.show_confirmation_dialog = true;
                }
            }

            if self.show_confirmation_dialog {
                egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui|{
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }
                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
            }
            
        });

        }
    }
    
}

pub fn main() -> Result<(), eframe::Error> {
    // env_logger::init();
    let mut controller = Controller::new(HashMap::new());
    let load_ini_file = load_ini_file();
    
    match load_ini_file {
        Ok(LoadResult::Success(json_file)) => {
            // let ret = LoadResult::Success(jsonfile);            
          
            let checkboxes = init_checkboxes(json_file["macros"].clone());
            controller = Controller::new(checkboxes);  
        },
        Ok(LoadResult::Error(err)) => {
            eprint!("Error reading file: {}", err);
            // Aqui você pode tomar outras ações específicas para o erro, se necessário
        },
        Err(err) => {
            eprint!("Error reading file: {}", err);
        },

    }
    
    let viewport = ViewportBuilder {
        inner_size: Some(Vec2{x:850., y:500.}),
        position: Some(Pos2{x:200., y:100.}),
        ..Default::default()
    };

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native( 
        "Biscuit - Docx 1.0",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc, controller)))
    )
}

fn button_main(ui: &mut Ui, counter: &mut i32, frame: &mut eframe::Frame) {
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}

