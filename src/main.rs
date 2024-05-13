mod file_explorer;

fn main() {
    let options = eframe::NativeOptions::default();

    let _ = eframe::run_native(
        "File Explorer",
        options,
        Box::new(|_cc| Box::new(file_explorer::FileExplorer::default())),
    );
}   

