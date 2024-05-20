#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App};
use home::home_dir;
use std::{
    fs, 
    path::PathBuf,
    os::windows::fs::MetadataExt, 
    // collections::HashMap,
    // sync::{Arc, Mutex, MutexGuard},
};

// use fs_extra;

pub struct FileExplorer {
    width: f32,
    height: f32,
    path: PathBuf,
    create_folder_dialog: bool,
    create_file_dialog: bool,
    delete_dialog: bool,
    show_hidden_files: bool,
    folder_name: String,
    file_name: String,
    selected_file: Option<PathBuf>,
    // sizes: Arc<Mutex<HashMap<PathBuf, u64>>>,
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
            path: (match home_dir() { Some(path) => path, None => PathBuf::new() }),
            create_folder_dialog: false,
            create_file_dialog: false,
            delete_dialog: false,
            show_hidden_files: false,
            folder_name: String::new(),
            file_name: String::new(),
            selected_file: None,
            // sizes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn calculate_window_size(width: f32, height: f32) -> f32 {
        match
            (width + height) / 90.0 // if the window is square
            {
                size if size < 20.0 => 20.0, // if the size is less than 20, set it to 20
                size => size, // otherwise, use the calculated size
            }
    }

    fn is_hidden_or_system(file_path: &PathBuf) -> Result<bool, std::io::Error> {
        let metadata: fs::Metadata = fs::metadata(file_path)?;
        let attributes = metadata.file_attributes();

        if (attributes & 0x2) > 0 || (attributes & 0x4) > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // fn get_size(&self, path: PathBuf, is_dir: bool, entry: &fs::DirEntry) -> u64 {
    //     let sizes: Arc<Mutex<HashMap<PathBuf, u64>>> = Arc::clone(&self.sizes);
    //     let size_option: Option<u64> = {
    //         let sizes_lock: MutexGuard<HashMap<PathBuf, u64>> = sizes.lock().unwrap();
    //         sizes_lock.get(&path).cloned()
    //     };

    //     if let Some(size) = size_option {
    //         size
    //     } else {
    //         let sizes: Arc<Mutex<HashMap<PathBuf, u64>>> = Arc::clone(&self.sizes);
    //         let path_clone: PathBuf = path.clone();
    //         let entry_metadata: fs::Metadata = entry.metadata().unwrap();

    //         let handle: thread::JoinHandle<u64> = thread::spawn(move || {
    //             let size: u64 = if is_dir {
    //                 let size: u64 = match fs_extra::dir::get_size(&path_clone) {
    //                     Ok(size) => size,
    //                     Err(_) => 0,
    //                 };
    //                 let mut sizes: MutexGuard<HashMap<PathBuf, u64>> = sizes.lock().unwrap();
    //                 sizes.insert(path_clone, size);
    //                 size
    //             } else {
    //                 entry_metadata.len()
    //             };
    //             size
    //         });

    //         handle.join().unwrap()
    //     }
    // }

    
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.width = ui.available_width();
        self.height = ui.available_height();
        println!("Showing Hidden Files: {}", self.show_hidden_files);
        egui::TopBottomPanel::top("top_panel").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("Current Path: {}", self.path.to_str().unwrap())).size(
                    Self::calculate_window_size(self.width, self.height)
                ));
                
                ui.separator();

                if ui.button(egui::RichText::new("Back").size(
                    Self::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.path.pop();
                }

                ui.separator();

                if ui.button(egui::RichText::new("Home").size(
                    Self::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.path = match home_dir() { Some(path) => path, None => PathBuf::new() };
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

                ui.separator();

                if ui.button(egui::RichText::new("Delete").size(
                    Self::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.delete_dialog = true;
                }

                ui.separator();

                // add a checkbox to show hidden and system files
                if ui.checkbox(&mut false, "Show Hidden and System Files").changed() {
                    self.show_hidden_files = !self.show_hidden_files;
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical()
            .drag_to_scroll(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
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
                
                ui.vertical(|ui| {
                    let entries: fs::ReadDir = fs::read_dir(&self.path).unwrap();
                    for entry in entries {
                        let entry: fs::DirEntry = entry.unwrap();
                        let path: PathBuf = entry.path();
                        let name: &str = path.file_name().unwrap().to_str().unwrap();

                        #[cfg(target_os = "windows")]
                        {
                            if !self.show_hidden_files && Self::is_hidden_or_system(&path).unwrap()
                                || !self.show_hidden_files && name.starts_with(".") {
                                continue;
                            }
                        }

                        #[cfg(not(target_os = "windows"))]
                        {
                            if !self.show_hidden_files && name.starts_with(".") {
                                continue;
                            }
                        }

                        let path_clone: PathBuf = path.clone();
                        let is_dir: bool = entry.metadata().unwrap().is_dir();

                        // let size: u64 = self.get_size(path.clone(), is_dir, &entry);   
                        let size: u64 = 0;                 

                        let file_type_str: &str = if is_dir { "Directory" } else { "File" };
                        ui.horizontal(|ui| {
                            let mut button = egui::Button::new(egui::RichText::new(name).size(
                                Self::calculate_window_size(self.width, self.height)
                            ));

                            if Some(path.clone()) == self.selected_file {
                                button = button.fill(egui::Color32::RED);
                            } else {
                                if file_type_str == "Directory" {
                                    // button = button.fill(egui::Color32::from_rgb(0, 0, 255));
                                    // gray not blue
                                    button = button.fill(egui::Color32::from_rgb(50, 50, 50));
                                } else {
                                    button = button.fill(egui::Color32::TRANSPARENT);
                                }
                            }

                            let response: egui::Response = ui.add(button);

                            if response.clicked_by(egui::PointerButton::Primary) {
                                if is_dir {
                                    if let Ok(_entries) = fs::read_dir(&path) {
                                        self.path = path_clone;
                                    }
                                } else {
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
                            } else if response.clicked_by(egui::PointerButton::Secondary) {
                                self.selected_file = Some(path_clone);
                                println!("Selected: {:?}", self.selected_file);
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
                                fs::create_dir(path).unwrap();
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
                                fs::File::create(path).unwrap();
                                self.file_name.clear();
                                self.create_file_dialog = false;
                            }
                        }
                    });
                });
        }

        let mut delete_dialog = self.delete_dialog;
        if delete_dialog {
            egui::Window::new("Delete File or Folder")
                .open(&mut delete_dialog)
                .show(ui.ctx(), |ui: &mut egui::Ui| {
                    ui.label("Are you sure you want to delete this file or folder?");
                    ui.horizontal(|ui: &mut egui::Ui| {
                        if ui.button("Cancel").clicked() {
                            self.delete_dialog = false;
                        }

                        ui.separator();

                        if ui.button("Delete").clicked() {
                            if let Some(selected_path) = &self.selected_file {
                                if selected_path.is_dir() {
                                    fs::remove_dir_all(selected_path).unwrap();
                                } else {
                                    fs::remove_file(selected_path).unwrap();
                                }
                                self.selected_file = None;
                            }
                            self.delete_dialog = false;
                        }
                    });
                });

            self.delete_dialog = delete_dialog;
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
    //     let mut path: PathBuf = home_dir().unwrap();
    //     path.push(".rust_explorer_cache");
    //     fs::write(path, cache).unwrap();
    // }
}