use serde::Serialize;

#[derive(Serialize)]
pub struct ExportData {
    pub res: String,
}

pub struct EngineInfo {
    name: String,
    cv: String,
    nm: String,
}

pub struct TransmissionData {
    name: String,
    speeds: String,
    retarder: bool,
    Ratio: String,
}
