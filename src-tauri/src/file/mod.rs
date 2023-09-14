use native_dialog::FileDialog;
use std::{fs, path::Path};

pub fn _choose_folder() -> Option<Vec<String>> {
    let path = FileDialog::new()
        .set_location("~/Pictures")
        .show_open_single_dir()
        .unwrap();

    let folder_path = fs::read_dir(path?).unwrap();

    let image_paths: Vec<String> = folder_path
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if is_image(&path) {
                Some(path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();

    if image_paths.is_empty() {
        None
    } else {
        Some(image_paths)
    }
}

fn is_image<P: AsRef<Path>>(path: P) -> bool {
    matches!(
        path.as_ref().extension().and_then(|s| s.to_str()),
        Some("jpg") | Some("jpeg") | Some("png")
    )
}
