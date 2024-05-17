#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};
use home;

mod utils;

pub struct FileExplorer {
    width: f32,
    height: f32,
    path: PathBuf,
    size_cache: Arc<Mutex<std::collections::HashMap<PathBuf, u64>>>,
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
            size_cache: (match home::home_dir() {
                Some(mut path) => {
                    path.push(".rust_explorer_cache");
                    if let Ok(cache) = std::fs::read(path) {
                        let cache: std::collections::HashMap<PathBuf, u64> = bincode::deserialize(&cache).unwrap();
                        Arc::new(Mutex::new(cache))
                    } else {
                        Arc::new(Mutex::new(std::collections::HashMap::new()))
                    }
                },
                None => Arc::new(Mutex::new(std::collections::HashMap::new())),
            }),
            create_folder_dialog: false,
            create_file_dialog: false,
            folder_name: String::new(),
            file_name: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.width = ui.available_width();
        self.height = ui.available_height();

        egui::ScrollArea::vertical()
        .drag_to_scroll(false)
        .show(ui, |ui: &mut egui::Ui| {
            ui.label(egui::RichText::new(format!("Current Path: {}", self.path.to_str().unwrap())).size(
                utils::calculate_window_size(self.width, self.height)
            ));
            ui.separator();

            ui.horizontal(|ui: &mut egui::Ui| {
                if ui.button(egui::RichText::new("Back").size(
                    utils::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.path.pop();
                }

                ui.separator();

                if ui.button(egui::RichText::new("Home").size(
                    utils::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.path = match home::home_dir() { Some(path) => path, None => PathBuf::new() };
                }

                ui.separator();

                if ui.button(egui::RichText::new("Create Folder").size(
                    utils::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.create_folder_dialog = true;
                }

                ui.separator();

                if ui.button(egui::RichText::new("Create File").size(
                    utils::calculate_window_size(self.width, self.height)
                )).clicked() {
                    self.create_file_dialog = true;
                }
            });

            ui.separator();

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("Name").size(
                    utils::calculate_window_size(self.width, self.height)
                ));
                ui.separator();
                ui.label(egui::RichText::new("Size").size(
                    utils::calculate_window_size(self.width, self.height)
                ));
                ui.separator();
                ui.label(egui::RichText::new("Type").size(
                    utils::calculate_window_size(self.width, self.height)
                ));
            });

            ui.separator();
            
            ui.vertical(|ui: &mut egui::Ui| {
                let entries: std::fs::ReadDir = std::fs::read_dir(&self.path).unwrap();
                for entry in entries {
                    let entry: std::fs::DirEntry = entry.unwrap();
                    let path: PathBuf = entry.path();
                    let name: &str = path.file_name().unwrap().to_str().unwrap();
                    
                    let size: u64 = *self.size_cache.lock().unwrap().entry(path.clone()).or_insert_with(|| {
                        let path_clone: PathBuf = path.clone();
                        thread::spawn(move || {
                            utils::calculate_directory_size(path_clone)
                        });
                        0
                    });

                    if size != 0 {
                        self.size_cache.lock().unwrap().insert(path.clone(), size);
                    }

                    let path_clone: PathBuf = path.clone();
                    let is_dir: bool = entry.metadata().unwrap().is_dir();
                    let file_type_str: &str = if is_dir { "Directory" } else { "File" };
                    ui.horizontal(|ui: &mut egui::Ui| {
                        if is_dir {
                            if ui.button(egui::RichText::new(name).size(
                                utils::calculate_window_size(self.width, self.height)
                            )).clicked() {
                                if let Ok(_entries) = std::fs::read_dir(&path) {
                                    self.path = path_clone;
                                }
                            }
                        } else {
                            let mut button: egui::Button = egui::Button::new(egui::RichText::new(name).size(
                                utils::calculate_window_size(self.width, self.height)
                            ));
                            button = button.frame(false);
                            if ui.add(button).clicked() {
                                utils::open_file(path_clone);
                            }
                        }

                        utils::format_sizes(size, ui, self.width, self.height);

                        ui.label(egui::RichText::new(file_type_str).size(
                            utils::calculate_window_size(self.width, self.height)
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

    fn on_exit(&mut self, _ctx: Option<&eframe::glow::Context>) {
        let cache: std::sync::MutexGuard<std::collections::HashMap<PathBuf, u64>> = self.size_cache.lock().unwrap();
        let cache: Vec<u8> = bincode::serialize(&*cache).unwrap();
        let mut path: PathBuf = home::home_dir().unwrap();
        path.push(".rust_explorer_cache");
        std::fs::write(path, cache).unwrap();
    }
}