// file for the exrtaneous functions I use in file_explorer.rs

use std::fs;
use std::path::PathBuf;
use eframe::egui;
use winapi;

pub fn calculate_directory_size(path: PathBuf) -> u64 {
    let mut total_size: u64 = 0;
    let mut stack: Vec<PathBuf> = vec![path.clone()];
    while let Some(dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let entry_path: PathBuf = entry.path();
                let size: u64 = entry.metadata().map(|m: fs::Metadata| m.len()).unwrap_or(0);
                total_size += size;
                if entry_path.is_dir() {
                    stack.push(entry_path);
                }
            }
        }
    }

    total_size
}

pub fn calculate_window_size(width: f32, height: f32) -> f32 {
    match
        (width + height) / 90.0 // if the window is square
        {
            size if size < 20.0 => 20.0, // if the size is less than 20, set it to 20
            size => size, // otherwise, use the calculated size
        }
}

pub fn open_file(path_clone: PathBuf) {
    // open the file with the default program for the file
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

pub fn format_sizes(size: u64, ui: &mut egui::Ui, width: f32, height: f32) {
    match size {
        0..=999 => ui.label(egui::RichText::new(format!("{} B", size)).size(
            calculate_window_size(width, height)
        )),
        1000..=999_999 => ui.label(egui::RichText::new(format!("{:.2} KB", size as f64 / 1000.0)).size(
            calculate_window_size(width, height)
        )),
        1_000_000..=999_999_999 => ui.label(egui::RichText::new(format!("{:.2} MB", size as f64 / 1_000_000.0)).size(
            calculate_window_size(width, height)
        )),
        1_000_000_000..=999_999_999_999 => ui.label(egui::RichText::new(format!("{:.2} GB", size as f64 / 1_000_000_000.0)).size(
            calculate_window_size(width, height)
        )),
        _ => ui.label(egui::RichText::new(format!("{:.2} TB", size as f64 / 1_000_000_000_000.0)).size(
            calculate_window_size(width, height)
        )),
    };
}