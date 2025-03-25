use choki::src::structs::ContentType;
use serde::{ Deserialize, Serialize };

use crate::utils::{ get_id_default };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub id: String,

    pub file_name: String,
    pub file_path: String,
    pub file_type: String,

    pub size: u64,

    pub upload_time: u64,
}
impl Image {
    pub fn new(
        file_name: &str,
        file_path: &str,
        file_type: &str,
        size: u64,
        upload_time: u64
    ) -> Image {
        return Image {
            id: get_id_default(),
            file_name: file_name.to_owned(),
            file_path: file_path.to_owned(),
            file_type: file_type.to_owned(),
            size: size,
            upload_time: upload_time,
        };
    }
}
