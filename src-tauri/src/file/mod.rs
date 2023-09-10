use native_dialog::{FileDialog, MessageDialog, MessageType};

pub fn layout() -> bool {
    let path = FileDialog::new()
        .set_location("~/Pictures")
        .show_open_single_dir()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return false,
    };

    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Do you want to open this folder?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();

    yes
}
