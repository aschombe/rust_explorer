#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App};
use home::home_dir;
use std::{fs, path::PathBuf};

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

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
            path: (match home_dir() {
                Some(path) => path,
                None => PathBuf::new(),
            }),
            create_folder_dialog: false,
            create_file_dialog: false,
            delete_dialog: false,
            show_hidden_files: false,
            folder_name: String::new(),
            file_name: String::new(),
            selected_file: None,
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

    #[cfg(target_os = "windows")]
    fn is_hidden_or_system(file_path: &PathBuf) -> Result<bool, std::io::Error> {
        let metadata: fs::Metadata = fs::metadata(file_path)?;
        let attributes = metadata.file_attributes();

        if (attributes & 0x2) > 0 || (attributes & 0x4) > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.width = ui.available_width();
        self.height = ui.available_height();

        egui::TopBottomPanel::top("top_panel").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("Current Path: {}", self.path.to_str().unwrap()))
                        .size(Self::calculate_window_size(self.width, self.height)),
                );

                ui.separator();

                if ui
                    .button(
                        egui::RichText::new("Back")
                            .size(Self::calculate_window_size(self.width, self.height)),
                    )
                    .clicked()
                {
                    self.path.pop();
                }

                ui.separator();

                if ui
                    .button(
                        egui::RichText::new("Home")
                            .size(Self::calculate_window_size(self.width, self.height)),
                    )
                    .clicked()
                {
                    self.path = match home_dir() {
                        Some(path) => path,
                        None => PathBuf::new(),
                    };
                }

                ui.separator();

                if ui
                    .button(
                        egui::RichText::new("Create Folder")
                            .size(Self::calculate_window_size(self.width, self.height)),
                    )
                    .clicked()
                {
                    self.create_folder_dialog = true;
                }

                ui.separator();

                if ui
                    .button(
                        egui::RichText::new("Create File")
                            .size(Self::calculate_window_size(self.width, self.height)),
                    )
                    .clicked()
                {
                    self.create_file_dialog = true;
                }

                ui.separator();

                if ui
                    .button(
                        egui::RichText::new("Delete")
                            .size(Self::calculate_window_size(self.width, self.height)),
                    )
                    .clicked()
                {
                    self.delete_dialog = true;
                }

                ui.separator();

                // add a checkbox to show hidden and system files
                if ui
                    .checkbox(&mut false, "Show Hidden and System Files")
                    .changed()
                {
                    self.show_hidden_files = !self.show_hidden_files;
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical()
                .drag_to_scroll(false)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Name")
                                .size(Self::calculate_window_size(self.width, self.height)),
                        );
                        ui.separator();
                        ui.label(
                            egui::RichText::new("Size")
                                .size(Self::calculate_window_size(self.width, self.height)),
                        );
                        ui.separator();
                        ui.label(
                            egui::RichText::new("Type")
                                .size(Self::calculate_window_size(self.width, self.height)),
                        );
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
                                if !self.show_hidden_files
                                    && Self::is_hidden_or_system(&path).unwrap()
                                    || !self.show_hidden_files && name.starts_with(".")
                                {
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

                            let size: u64 = entry.metadata().unwrap().len();

                            let file_type_str: &str = if is_dir { "Directory" } else { "File" };
                            ui.horizontal(|ui| {
                                let mut button = egui::Button::new(
                                    egui::RichText::new(name)
                                        .size(Self::calculate_window_size(self.width, self.height)),
                                );

                                if Some(path.clone()) == self.selected_file {
                                    button = button.fill(egui::Color32::RED);
                                } else {
                                    if file_type_str == "Directory" {
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
                                        open::that(path_clone).unwrap();
                                    }
                                } else if response.clicked_by(egui::PointerButton::Secondary) {
                                    self.selected_file = Some(path_clone);
                                    println!("Selected: {:?}", self.selected_file);
                                }

                                match size {
                                    0..=999 => {
                                        ui.label(egui::RichText::new(format!("{} B", size)).size(
                                            Self::calculate_window_size(self.width, self.height),
                                        ))
                                    }
                                    1000..=999_999 => ui.label(
                                        egui::RichText::new(format!(
                                            "{:.2} KB",
                                            size as f64 / 1000.0
                                        ))
                                        .size(Self::calculate_window_size(self.width, self.height)),
                                    ),
                                    1_000_000..=999_999_999 => ui.label(
                                        egui::RichText::new(format!(
                                            "{:.2} MB",
                                            size as f64 / 1_000_000.0
                                        ))
                                        .size(Self::calculate_window_size(self.width, self.height)),
                                    ),
                                    1_000_000_000..=999_999_999_999 => ui.label(
                                        egui::RichText::new(format!(
                                            "{:.2} GB",
                                            size as f64 / 1_000_000_000.0
                                        ))
                                        .size(Self::calculate_window_size(self.width, self.height)),
                                    ),
                                    _ => ui.label(
                                        egui::RichText::new(format!(
                                            "{:.2} TB",
                                            size as f64 / 1_000_000_000_000.0
                                        ))
                                        .size(Self::calculate_window_size(self.width, self.height)),
                                    ),
                                };

                                ui.label(
                                    egui::RichText::new(file_type_str)
                                        .size(Self::calculate_window_size(self.width, self.height)),
                                );
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
}
