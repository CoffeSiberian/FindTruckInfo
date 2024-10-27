mod strucs;

use serde_json::{to_string, to_string_pretty};
use std::fs::{read_dir, write, File};
use std::io::Read;
use std::path::PathBuf;
use strucs::export_data::{EngineInfo, ExportData, TransmissionData};
use strucs::file_data::{FileData, FolderData};

fn read_file(path: &PathBuf) -> Option<String> {
    let file_open = File::open(path);

    let mut file = match file_open {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut buffer: String = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(_) => return None,
    };

    return Some(buffer);
}

fn read_file_text_vec(path: &PathBuf) -> Option<Vec<String>> {
    let file = match read_file(path) {
        Some(file) => file,
        None => return None,
    };

    return Some(file.lines().map(|s| s.to_string()).collect());
}

fn save_as_json(data: Vec<ExportData>, path: &str, pretty_file: bool) -> bool {
    let json_data = match if pretty_file {
        to_string_pretty(&data)
    } else {
        to_string(&data)
    } {
        Ok(json_data) => json_data,
        Err(_) => return false,
    };

    match write(path, json_data) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn list_files(path: &PathBuf) -> Option<(Vec<FileData>, usize)> {
    let entries = match read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    let mut files: Vec<FileData> = Vec::new();

    for entry in entries {
        let entry_data = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let file_extension = match entry_data.path().extension() {
            Some(extension) => extension.to_string_lossy().to_string(),
            None => continue,
        };

        if file_extension != "sii" {
            continue;
        }

        let file_path = entry_data.path();
        let file_name = match entry_data.file_name().into_string() {
            Ok(file_name) => file_name,
            Err(_) => continue,
        };

        files.push(FileData {
            path: file_path,
            file_name,
        });
    }

    if files.is_empty() {
        return None;
    }

    let total_files = files.len();

    return Some((files, total_files));
}

fn list_folders(path: &PathBuf) -> Option<Vec<FolderData>> {
    let entries = match read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    let mut folders: Vec<FolderData> = Vec::new();

    for entry in entries {
        let entry_data = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if !entry_data.path().is_dir() {
            continue;
        }

        let folder_path = entry_data.path();
        let folder_name = match entry_data.file_name().into_string() {
            Ok(folder_name) => folder_name,
            Err(_) => continue,
        };

        folders.push(FolderData {
            folder_name,
            path: folder_path,
        });
    }

    if folders.is_empty() {
        return None;
    }

    return Some(folders);
}

fn read_engine_data(path: &PathBuf) -> Option<Vec<EngineInfo>> {
    let path_engines = path.join("engine");

    let (list_files_engines, total_files) = match list_files(&path_engines) {
        Some(list_files) => list_files,
        None => return None,
    };

    let mut data: Vec<EngineInfo> = Vec::new();

    for files in list_files_engines {
        println!("Reading file: {}", files.file_name);
    }

    return None;
}

fn read_transmission_data(path: &PathBuf) -> Option<Vec<TransmissionData>> {
    let path_transmissions = path.join("transmission");

    let (list_files_transmissions, total_files) = match list_files(&path_transmissions) {
        Some(list_files) => list_files,
        None => return None,
    };

    for files in list_files_transmissions {
        println!("Reading file: {}", files.file_name);
    }

    return None;
}

fn main() {
    let path = PathBuf::from("path");

    let list_folders_trucks = match list_folders(&path) {
        Some(list_folders) => list_folders,
        None => return,
    };

    for folder in list_folders_trucks {
        read_engine_data(&folder.path);
        read_transmission_data(&folder.path);
    }
}
