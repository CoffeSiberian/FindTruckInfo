use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct BrandData {
    pub model: String,
    pub engines: Vec<EngineInfo>,
    pub transmissions: Vec<TransmissionData>,
}

#[derive(Serialize, Clone)]
pub struct EngineInfo {
    pub name: String,
    pub cv: String,
    pub nm: String,
    pub code: String,
}

#[derive(Serialize, Clone)]
pub struct TransmissionData {
    pub name: String,
    pub speeds: String,
    pub retarder: bool,
    pub ratio: String,
    pub code: String,
}
