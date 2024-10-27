mod strucs;

use serde_json::{to_string, to_string_pretty};
use std::fs::{read_dir, write, File};
use std::io::Read;
use std::path::PathBuf;
use strucs::export_data::{EngineInfo, ExportData, TransmissionData};
use strucs::file_data::{FileData, FolderData};

fn read_file(path: &PathBuf) -> Option<File> {
    let file = File::open(path);

    match file {
        Ok(file) => return Some(file),
        Err(_) => return None,
    };
}

fn file_split_space(path: &PathBuf) -> Option<Vec<String>> {
    let mut file = match read_file(path) {
        Some(file) => file,
        None => return None,
    };

    let mut buf_reader: String = String::new();
    match file.read_to_string(&mut buf_reader) {
        Ok(_) => (),
        Err(_) => return None,
    }

    if buf_reader.contains("\r\n") {
        // CR LF
        return Some(buf_reader.split("\r\n").map(|s| s.to_owned()).collect());
    } else if buf_reader.contains("\n") {
        // LF
        return Some(buf_reader.split("\n").map(|s| s.to_owned()).collect());
    }
    return None;
}

#[allow(dead_code)]
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

fn get_object_name(name: &String) -> Option<String> {
    let split: Vec<&str> = name.split('"').collect();

    if split.len() > 1 {
        let split_name = split[1];

        if split_name.contains("@@kw@@") {
            let split_name_kw = split_name.replace("@@kw@@", "kw");

            return Some(split_name_kw.to_string());
        }

        return Some(split_name.to_string());
    }

    return None;
}

fn get_engine_cv(name: &String) -> Option<String> {
    let split: Vec<&str> = name.split('"').collect();

    if split.len() > 1 {
        let split_cv: Vec<&str> = split[1].split_whitespace().collect();

        if split_cv.len() > 1 {
            return Some(split_cv[0].to_string());
        }
    }

    return None;
}

fn read_engine_file(path: &PathBuf) -> Option<EngineInfo> {
    let file = match file_split_space(path) {
        Some(file) => file,
        None => return None,
    };

    let mut name: String = String::new();
    let mut cv: String = String::new();
    let mut nm: String = String::new();
    let mut rpm: String = String::new();

    for line in file {
        if line.contains("name:") {
            name = match get_object_name(&line) {
                Some(name) => name,
                None => continue,
            };
        }

        if line.contains("info[]:") && cv.is_empty() {
            cv = match get_engine_cv(&line) {
                Some(cv) => cv,
                None => continue,
            };
        }

        if line.contains("torque:") {
            let torque_split: Vec<&str> = line.split(':').collect();

            if torque_split.len() > 1 {
                nm = torque_split[1].trim().to_string();
            }
        }

        if line.contains("rpm_limit:") {
            let rpm_split: Vec<&str> = line.split(':').collect();

            if rpm_split.len() > 1 {
                rpm = rpm_split[1].trim().to_string();
            }
        }
    }

    if name.is_empty() || cv.is_empty() || nm.is_empty() || rpm.is_empty() {
        return None;
    }

    return Some(EngineInfo { name, cv, nm, rpm });
}

fn list_engine_data(path: &PathBuf) -> Option<Vec<EngineInfo>> {
    let path_engines = path.join("engine");

    let (list_files_engines, total_files) = match list_files(&path_engines) {
        Some(list_files) => list_files,
        None => return None,
    };

    let mut data: Vec<EngineInfo> = Vec::new();

    for files in list_files_engines {
        let engine_data = match read_engine_file(&files.path) {
            Some(engine_data) => engine_data,
            None => continue,
        };

        println!("Engine: {}", engine_data.name);
        data.push(engine_data);
    }

    return None;
}

fn list_transmission_data(path: &PathBuf) -> Option<Vec<TransmissionData>> {
    let path_transmissions = path.join("transmission");

    let (list_files_transmissions, total_files) = match list_files(&path_transmissions) {
        Some(list_files) => list_files,
        None => return None,
    };

    for files in list_files_transmissions {
        //println!("Reading file: {}", files.file_name);
    }

    return None;
}

fn main() {
    let path = PathBuf::from("C:/Users/coffe/Desktop/SCS Extrac/truck");

    let list_folders_trucks = match list_folders(&path) {
        Some(list_folders) => list_folders,
        None => return,
    };

    for folder in list_folders_trucks {
        list_engine_data(&folder.path);
        list_transmission_data(&folder.path);
    }
}
