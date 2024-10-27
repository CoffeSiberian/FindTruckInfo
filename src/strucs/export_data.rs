use serde::Serialize;

#[derive(Serialize)]
pub struct ExportData {
    pub res: String,
}

pub struct EngineInfo {
    pub name: String,
    pub cv: String,
    pub nm: String,
    pub rpm: String,
}

pub struct TransmissionData {
    pub name: String,
    pub speeds: String,
    pub retarder: bool,
    pub Ratio: String,
}
