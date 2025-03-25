use std::{ env, time::{ Instant, SystemTime, UNIX_EPOCH } };

use bson::doc;
use mongodb::{ bson::oid::ObjectId };
use mongodb::sync::{ Client, Collection };

use crate::{ structs::Image, utils::{ self, get_id } };

#[derive(Clone)]
pub struct Database {
    client: Client,
    images: Collection<Image>,
}

impl Database {
    pub fn new() -> Database {
        let client_uri = env
            ::var("MONGODB_URI")
            .expect("You must set the MONGODB_URI environment var!");

        let client = Client::with_uri_str(client_uri).unwrap();

        let images = client.database("Roti").collection::<Image>("Images");

        return Database { client: client, images: images };
    }
    pub fn add_image(&self, image: Image) -> bool {
        match self.images.insert_one(image).run() {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                return false;
            }
        }
    }
    pub fn get_image(&self, id: &str) -> Option<Image> {
        let task = self.images.find_one(doc! { "id": id });
        task.run().unwrap_or_default()
    }
}
