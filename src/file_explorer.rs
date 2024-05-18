#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App};
use std::{
    // fs,
    // thread,
    path::PathBuf,
    // sync::{Arc, Mutex},
};
use home;


pub struct FileExplorer {
    width: f32,
    height: f32,
    path: PathBuf,
    // size_cache: Arc<Mutex<std::collections::HashMap<PathBuf, u64>>>,
    create_folder_dialog: bool,
    create_file_dialog: bool,
    folder_name: String,
    file_name: String,
}

impl Default for FileExplorer {
    fn default() -> Self {
        Self::new()
    }
}

impl FileExplorer {
    pub fn new() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            // set path to the home directory for the current user, windows and unix
            path: (match home::home_dir() { Some(path) => path, None => PathBuf::new() }),
            // if it exists, load it, otherwise create a new one (gets saved on exit)
            // size_cache: (match home::home_dir() {
            //     Some(mut path) => {
            //         path.push(".rust_explorer_cache");
            //         if let Ok(cache) = std::fs::read(path) {
            //             let cache: std::collections::HashMap<PathBuf, u64> = bincode::deserialize(&cache).unwrap();
            //             Arc::new(Mutex::new(cache))
            //         } else {
            //             Arc::new(Mutex::new(std::collections::HashMap::new()))
            //         }
            //     },
            //     None => Arc::new(Mutex::new(std::collections::HashMap::new())),
            // }),
            create_folder_dialog: false,
            create_file_dialog: false,
            folder_name: String::new(),
            file_name: String::new(),
        }
    }

    pub fn calculate_window_size(width: f32, height: f32) -> f32 {
        match
            (width + height) / 90.0 // if the window is square
            {
                size if size < 20.0 => 20.0, // if the size is less than 20, set it to 20
                size => size, // otherwise, use the calculated size
            }
    }

    // pub fn calculate_directory_size(path: PathBuf) -> u64 {
    //     let mut total_size: u64 = 0;
    //     let mut stack: Vec<PathBuf> = vec![path.clone()];
    //     while let Some(dir) = stack.pop() {
    //         if let Ok(entries) = fs::read_dir(&dir) {
    //             for entry in entries.flatten() {
    //                 let entry_path: PathBuf = entry.path();
    //                 let size: u64 = entry.metadata().map(|m: fs::Metadata| m.len()).unwrap_or(0);
    //                 total_size += size;
    //                 if entry_path.is_dir() {
    //                     stack.push(entry_path);
    //                 }
    //             }
    //         }
    //     }
    
    //     total_size
    // }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.width = ui.available_width();
        self.height = ui.available_height();

        egui::ScrollArea::vertical()
        .drag_to_scroll(false)
        .show(ui, |ui: &mut egui::Ui| {
            ui.label(egui::RichText::new(format!("Current Path: {}", self.path.to_str().unwrap())).size(
                Self::calculate_window_size(self.width, self.height)
            ));
            ui.separator();

            ui.horizontal(|ui: &mut egui::Ui| {
                if ui.button(egui::RichText::new("Back").size(
                    Self::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.path.pop();
                }

                ui.separator();

                if ui.button(egui::RichText::new("Home").size(
                    Self::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.path = match home::home_dir() { Some(path) => path, None => PathBuf::new() };
                }

                ui.separator();

                if ui.button(egui::RichText::new("Create Folder").size(
                    Self::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.create_folder_dialog = true;
                }

                ui.separator();

                if ui.button(egui::RichText::new("Create File").size(
                    Self::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.create_file_dialog = true;
                }
            });

            ui.separator();

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("Name").size(
                    Self::calculate_window_size(self.width, self.height)
                ));
                ui.separator();
                ui.label(egui::RichText::new("Size").size(
                    Self::calculate_window_size(self.width, self.height)
                ));
                ui.separator();
                ui.label(egui::RichText::new("Type").size(
                    Self::calculate_window_size(self.width, self.height)
                ));
            });

            ui.separator();
            
            ui.vertical(|ui: &mut egui::Ui| {
                let entries: std::fs::ReadDir = std::fs::read_dir(&self.path).unwrap();
                for entry in entries {
                    let entry: std::fs::DirEntry = entry.unwrap();
                    let path: PathBuf = entry.path();
                    let name: &str = path.file_name().unwrap().to_str().unwrap();
                    
                    // let size: u64 = *self.size_cache.lock().unwrap().entry(path.clone()).or_insert_with(|| {
                    //     let path_clone: PathBuf = path.clone();
                    //     thread::spawn(move || {
                    //         Self::calculate_directory_size(path_clone)
                    //     });
                    //     0
                    // });

                    // if size != 0 {
                    //     self.size_cache.lock().unwrap().insert(path.clone(), size);
                    // }

                    let size: u64 = 0;

                    let path_clone: PathBuf = path.clone();
                    let is_dir: bool = entry.metadata().unwrap().is_dir();
                    let file_type_str: &str = if is_dir { "Directory" } else { "File" };
                    ui.horizontal(|ui: &mut egui::Ui| {
                        if is_dir {
                            if ui.button(egui::RichText::new(name).size(
                                Self::calculate_window_size(self.width, self.height)
                            )).clicked() {
                                if let Ok(_entries) = std::fs::read_dir(&path) {
                                    self.path = path_clone;
                                }
                            }
                        } else {
                            let mut button: egui::Button = egui::Button::new(egui::RichText::new(name).size(
                                Self::calculate_window_size(self.width, self.height)
                            ));
                            button = button.frame(false);
                            if ui.add(button).clicked() {
                                #[cfg(target_os = "windows")]
                                {
                                    // use winapi to open the default program for the file
                                    let path_str: &str = path_clone.to_str().unwrap();
                                    let path_str: std::ffi::CString = std::ffi::CString::new(path_str).unwrap();
                                    unsafe {
                                        winapi::um::shellapi::ShellExecuteA(
                                            std::ptr::null_mut(),
                                            std::ptr::null_mut(),
                                            path_str.as_ptr(),
                                            std::ptr::null(),
                                            std::ptr::null(),
                                            winapi::um::winuser::SW_SHOWNORMAL,
                                        );
                                    }
                                }
                                // try to open the file with their default program using xdg-open, 
                                // otherwise use the default editor, or nano if the default editor is not set
                                #[cfg(not(target_os = "windows"))]
                                {
                                    // let editor = std::env::var("EDITOR").unwrap_or("nano".to_string());
                                    // std::process::Command::new(editor)
                                    //     .arg(path_clone)
                                    //     .spawn()
                                    //     .unwrap();
                                    
                                    // mimeapps.list
                            
                                    let path_str = path_clone.to_str().unwrap();
                                    let path_str = std::ffi::CString::new(path_str).unwrap();
                                    if let Err(_) = std::process::Command::new("xdg-open")
                                        .arg(path_str)
                                        .spawn()
                                    {
                                        let editor = std::env::var("EDITOR").unwrap_or("nano".to_string());
                                        std::process::Command::new(editor)
                                            .arg(path_clone)
                                            .spawn()
                                            .unwrap();
                                    }
                                }
                            }
                        }

                        match size {
                            0..=999 => ui.label(egui::RichText::new(format!("{} B", size)).size(
                                Self::calculate_window_size(self.width, self.height)
                            )),
                            1000..=999_999 => ui.label(egui::RichText::new(format!("{:.2} KB", size as f64 / 1000.0)).size(
                                Self::calculate_window_size(self.width, self.height)
                            )),
                            1_000_000..=999_999_999 => ui.label(egui::RichText::new(format!("{:.2} MB", size as f64 / 1_000_000.0)).size(
                                Self::calculate_window_size(self.width, self.height)
                            )),
                            1_000_000_000..=999_999_999_999 => ui.label(egui::RichText::new(format!("{:.2} GB", size as f64 / 1_000_000_000.0)).size(
                                Self::calculate_window_size(self.width, self.height)
                            )),
                            _ => ui.label(egui::RichText::new(format!("{:.2} TB", size as f64 / 1_000_000_000_000.0)).size(
                                Self::calculate_window_size(self.width, self.height)
                            )),
                        };

                        ui.label(egui::RichText::new(file_type_str).size(
                            Self::calculate_window_size(self.width, self.height)
                        ));
                    });
                }
            });
        });

        let mut create_folder_dialog = self.create_folder_dialog;
        if create_folder_dialog {
            egui::Window::new("Create Folder")
                .open(&mut create_folder_dialog)
                .show(ui.ctx(), |ui: &mut egui::Ui| {
                    ui.horizontal(|ui: &mut egui::Ui| {
                        ui.label("Folder Name:");
                        ui.text_edit_singleline(&mut self.folder_name);
                    });

                    ui.horizontal(|ui: &mut egui::Ui| {
                        if ui.button("Cancel").clicked() {
                            self.create_folder_dialog = false;
                        }

                        ui.separator();

                        if ui.button("Create").clicked() {
                            if !self.folder_name.is_empty() {
                                let mut path = self.path.clone();
                                path.push(&self.folder_name);
                                std::fs::create_dir(path).unwrap();
                                self.folder_name.clear();
                                self.create_folder_dialog = false;
                            }
                        }
                    });
                });
        }

        let mut create_folder_dialog = self.create_folder_dialog;
        if create_folder_dialog {
            egui::Window::new("Create File")
                .open(&mut create_folder_dialog)
                .show(ui.ctx(), |ui: &mut egui::Ui| {
                    ui.horizontal(|ui: &mut egui::Ui| {
                        ui.label("File Name:");
                        ui.text_edit_singleline(&mut self.file_name);
                    });

                    ui.horizontal(|ui: &mut egui::Ui| {
                        if ui.button("Cancel").clicked() {
                            self.create_file_dialog = false;
                        }

                        ui.separator();

                        if ui.button("Create").clicked() {
                            if !self.file_name.is_empty() {
                                let mut path = self.path.clone();
                                path.push(&self.file_name);
                                std::fs::File::create(path).unwrap();
                                self.file_name.clear();
                                self.create_file_dialog = false;
                            }
                        }
                    });
                });
        }
    }
}

impl App for FileExplorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            self.ui(ui);
        });
    }

    // fn on_exit(&mut self, _ctx: Option<&eframe::glow::Context>) {
    //     let cache: std::sync::MutexGuard<std::collections::HashMap<PathBuf, u64>> = self.size_cache.lock().unwrap();
    //     let cache: Vec<u8> = bincode::serialize(&*cache).unwrap();
    //     let mut path: PathBuf = home::home_dir().unwrap();
    //     path.push(".rust_explorer_cache");
    //     std::fs::write(path, cache).unwrap();
    // }
}