use std::path::PathBuf;

pub struct FileData {
    pub path: PathBuf,
    pub file_name: String,
}

pub struct FolderData {
    pub path: PathBuf,
    pub folder_name: String,
}
