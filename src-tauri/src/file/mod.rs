use native_dialog::FileDialog;
use std::{fs, path::Path};

pub fn _choose_folder() -> Option<String> {
    // for now returns first image from folder
    let path = FileDialog::new()
        .set_location("~/Pictures")
        .show_open_single_dir()
        .unwrap();

    let folder_path = fs::read_dir(path?).unwrap();

    for file_path_result in folder_path.flatten() {
        if is_image(&file_path_result.path()) {
            return Some(file_path_result.path().to_string_lossy().into_owned());
        }
    }
    None
}

fn is_image<P: AsRef<Path>>(path: P) -> bool {
    matches!(
        path.as_ref().extension().and_then(|s| s.to_str()),
        Some("jpg") | Some("jpeg") | Some("png")
    )
}
