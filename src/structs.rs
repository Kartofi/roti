use bson::{ doc, Document };
use choki::src::structs::ContentType;
use serde::{ Deserialize, Serialize };

use crate::utils::{ get_id_default };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ban {
    pub ip: String,
    pub reason: String,
    pub time: i64,
}
impl Ban {
    pub fn new() -> Ban {
        Ban { ip: "".to_string(), reason: "".to_string(), time: 0 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub ip: String,
    pub user_agent: String,
}
impl User {
    pub fn new() -> User {
        User { ip: "".to_string(), user_agent: "".to_string() }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub id: String,

    pub file_name: String,
    pub file_path: String,
    pub file_type: String,

    pub size: u64,
    pub views: u64,
    pub uploader: User,

    pub upload_time: i64,
}
impl Image {
    pub fn new() -> Image {
        return Image {
            id: get_id_default(),
            file_name: "".to_string(),
            file_path: "".to_string(),
            file_type: "".to_string(),
            size: 0,
            views: 0,
            uploader: User::new(),
            upload_time: 0,
        };
    }
    pub fn to_bson_simple(&self) -> Document {
        let size_i64 = self.size as i64;
        let views_i64 = self.views as i64;

        let doc =
            doc! {
            "id": &self.id,
            "file_name": &self.file_name,
            "file_type": &self.file_type,
            "size": size_i64,
            "views":views_i64,
            "upload_time": &self.upload_time
        };
        return doc;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub id: String,

    pub ip: String,
    pub expire_time: i64,
}
impl Session {
    pub fn new() -> Session {
        Session { id: "".to_string(), ip: "".to_string(), expire_time: 0 }
    }
}
