mod strucs;

use serde_json::{to_string, to_string_pretty};
use std::collections::HashMap;
use std::fs::{read_dir, write, File};
use std::io::Read;
use std::path::PathBuf;
use strucs::export_data::{BrandData, EngineInfo, TransmissionData};
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

fn save_as_json(data: HashMap<String, Vec<BrandData>>, path: &str, pretty_file: bool) -> bool {
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

fn list_files(path: &PathBuf) -> Option<Vec<FileData>> {
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

    return Some(files);
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

fn get_normal_value(name: &String) -> Option<String> {
    let split: Vec<&str> = name.split(':').collect();

    if split.len() > 1 {
        return Some(split[1].trim().to_string());
    }

    return None;
}

fn get_brand(data: &String) -> Option<String> {
    let split_name = data.split(".").collect::<Vec<&str>>();

    if split_name.len() > 1 {
        return Some(split_name[0].to_string());
    }

    return None;
}

fn get_model(data: &String) -> Option<String> {
    let split_name = data.split(".").collect::<Vec<&str>>();

    if split_name.len() > 1 {
        return Some(split_name[1].to_string());
    }

    return None;
}

fn read_engine_file(
    path: &PathBuf,
    folder_name: &String,
    file_name: &String,
) -> Option<EngineInfo> {
    let file = match file_split_space(path) {
        Some(file) => file,
        None => return None,
    };

    let mut name: String = String::new();
    let mut cv: String = String::new();
    let mut nm: String = String::new();

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
            nm = match get_normal_value(&line) {
                Some(nm) => nm,
                None => continue,
            };
        }
    }

    if name.is_empty() || cv.is_empty() || nm.is_empty() {
        return None;
    }

    let code = format!("/def/vehicle/truck/{}/engine/{}", folder_name, file_name);

    return Some(EngineInfo { name, cv, nm, code });
}

fn get_transmission_ratio(file: Vec<String>, i_start: usize, i_end: usize) -> Option<String> {
    let ratio_1 = match get_normal_value(&file[i_start]) {
        Some(ratio) => ratio,
        None => return None,
    };
    let ratio_2 = match get_normal_value(&file[i_end]) {
        Some(ratio) => ratio,
        None => return None,
    };

    return Some(format!("{} - {}", ratio_1, ratio_2));
}

fn read_transmission_file(
    path: &PathBuf,
    folder_name: &String,
    file_name: &String,
) -> Option<TransmissionData> {
    let file = match file_split_space(path) {
        Some(file) => file,
        None => return None,
    };

    let mut name: String = String::new();
    let mut speeds: u8 = 0;
    let mut retarder: bool = false;

    let mut ratio_index_start: usize = 0;
    let mut ratio_index_end: usize = 0;

    for (i, line) in file.iter().enumerate() {
        if line.contains("name:") && name.is_empty() {
            name = match get_object_name(&line) {
                Some(name) => name,
                None => continue,
            };
        }

        if line.contains("ratios_forward[") {
            if ratio_index_start == 0 {
                ratio_index_start = i
            } else {
                ratio_index_end = i;
            }

            speeds += 1;
        }

        if line.contains("retarder:") {
            retarder = true;
        }
    }

    if ratio_index_start == 0 || ratio_index_end == 0 {
        return None;
    }

    let ratio = match get_transmission_ratio(file, ratio_index_start, ratio_index_end) {
        Some(ratio) => ratio,
        None => return None,
    };

    if name.is_empty() || speeds == 0 {
        return None;
    }

    let code = format!(
        "/def/vehicle/truck/{}/transmission/{}",
        folder_name, file_name
    );

    return Some(TransmissionData {
        name,
        speeds: speeds.to_string(),
        retarder,
        ratio,
        code,
    });
}

fn list_engine_data(path: &PathBuf, folder_name: &String) -> Option<Vec<EngineInfo>> {
    let path_engines = path.join("engine");

    let list_files_engines = match list_files(&path_engines) {
        Some(list_files) => list_files,
        None => return None,
    };

    let mut data: Vec<EngineInfo> = Vec::new();

    for files in list_files_engines {
        match read_engine_file(&files.path, &folder_name, &files.file_name) {
            Some(engine_data) => data.push(engine_data),
            None => continue,
        };
    }

    if data.is_empty() {
        return None;
    }

    return Some(data);
}

fn list_transmission_data(path: &PathBuf, folder_name: &String) -> Option<Vec<TransmissionData>> {
    let path_transmissions = path.join("transmission");

    let list_files_transmissions = match list_files(&path_transmissions) {
        Some(list_files) => list_files,
        None => return None,
    };

    let mut data: Vec<TransmissionData> = Vec::new();

    for files in list_files_transmissions {
        match read_transmission_file(&files.path, &folder_name, &files.file_name) {
            Some(transmission_data) => data.push(transmission_data),
            None => continue,
        };
    }

    if data.is_empty() {
        return None;
    }

    return Some(data);
}

fn main() {
    let path = PathBuf::from("path");

    let list_folders_trucks = match list_folders(&path) {
        Some(list_folders) => list_folders,
        None => return,
    };

    let mut data_raw_trucks: Vec<BrandData> = Vec::new();

    for folder in list_folders_trucks {
        let engines = match list_engine_data(&folder.path, &folder.folder_name) {
            Some(engines) => engines,
            None => continue,
        };

        let transmissions = match list_transmission_data(&folder.path, &folder.folder_name) {
            Some(transmission) => transmission,
            None => continue,
        };

        let brand = match get_brand(&folder.folder_name) {
            Some(brand) => brand,
            None => continue,
        };

        let model = match get_model(&folder.folder_name) {
            Some(model) => model,
            None => continue,
        };

        data_raw_trucks.push(BrandData {
            brand,
            model: model,
            engines,
            transmissions,
        });
    }

    let mut data: HashMap<String, Vec<BrandData>> = HashMap::new();

    for truck_data in &data_raw_trucks {
        data.entry(truck_data.brand.clone())
            .or_insert(Vec::new())
            .push(truck_data.clone());
    }

    let path_json = "path.json";
    save_as_json(data, path_json, true);
}
