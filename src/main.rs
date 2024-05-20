mod file_explorer;


fn main() {
    let mut options: eframe::NativeOptions = eframe::NativeOptions::default();
    options.follow_system_theme = true;

    let _ = eframe::run_native(
        "File Explorer",
        options,
        Box::new(|_cc: &eframe::CreationContext| Box::new(file_explorer::FileExplorer::default())),
    );
}