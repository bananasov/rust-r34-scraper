use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub response: Vec<File>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub id: i64,
    pub score: i64,
    pub high_res_file: HighResFile,
    pub low_res_file: LowResFile,
    pub tags: Vec<String>,
    pub rating: String,
    pub media_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HighResFile {
    pub url: String,
    pub width: i64,
    pub height: i64
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LowResFile {
    pub url: String,
    pub width: i64,
    pub height: i64
}