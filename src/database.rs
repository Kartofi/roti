use std::{ env, fs, time::{ Instant, SystemTime, UNIX_EPOCH } };

use bson::doc;
use mongodb::{ bson::oid::ObjectId, results::DeleteResult };
use mongodb::sync::{ Client, Collection };

use crate::{
    routes::image,
    structs::{ Ban, Image, Session, User },
    utils::{ self, get_id, get_timestamp },
    SESSION_EXPIRE_TIME,
    SESSION_ID_LENGTH,
};

#[derive(Clone)]
pub struct Database {
    client: Client,
    images: Collection<Image>,
    banned_users: Collection<Ban>,
    admin_sessions: Collection<Session>,
}

impl Database {
    pub fn new() -> Database {
        let client_uri = env
            ::var("MONGODB_URI")
            .expect("You must set the MONGODB_URI environment var!");

        let client = Client::with_uri_str(client_uri).unwrap();

        let images = client.database("Roti").collection::<Image>("Images");
        let banned_users = client.database("Roti").collection::<Ban>("Banned_Users");
        let admin_sessions = client.database("Roti").collection::<Session>("Admin_Sessions");

        return Database {
            client: client,
            images: images,
            banned_users: banned_users,
            admin_sessions: admin_sessions,
        };
    }
    //Admin
    pub fn get_session(&self, id: &str) -> Option<Session> {
        let task = self.admin_sessions.find_one(doc! { "id":id });
        match task.run() {
            Ok(res) => {
                return res;
            }
            Err(_) => {
                return None;
            }
        }
    }
    pub fn remove_session(&self, id: &str) -> bool {
        let session = self.get_session(id);
        if session.is_none() {
            return false;
        }

        let task = self.admin_sessions.delete_one(doc! { "id": id });
        match task.run() {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                return false;
            }
        }
    }
    pub fn add_session(&self, session: Session) -> Option<Session> {
        match self.admin_sessions.insert_one(&session).run() {
            Ok(_) => {
                return Some(session);
            }
            Err(_) => {
                return None;
            }
        }
    }

    //Statistics
    pub fn total_bans(&self) -> usize {
        let task = self.banned_users.find(doc! {});
        let count = task
            .run()
            .unwrap()
            .filter_map(|doc| {
                doc.ok().map(|d| {
                    return d;
                })
            })
            .count();
        return count;
    }
    pub fn total_views(&self) -> u64 {
        let task = self.images.find(doc! {});
        let sizes: Vec<u64> = task
            .run()
            .unwrap()
            .filter_map(|doc| {
                doc.ok().map(|d| {
                    return d.views;
                })
            })
            .collect();
        return sizes.iter().sum();
    }
    pub fn total_size(&self) -> u64 {
        let task = self.images.find(doc! {});
        let sizes: Vec<u64> = task
            .run()
            .unwrap()
            .filter_map(|doc| {
                doc.ok().map(|d| {
                    return d.size;
                })
            })
            .collect();
        return sizes.iter().sum();
    }
    //Images
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
    pub fn delete_image(&self, id: &str) -> bool {
        let image = self.get_image(id);
        if image.is_none() {
            return false;
        }
        let image = image.unwrap();
        fs::remove_file(&image.file_path).unwrap_or_default();

        let task = self.images.delete_one(doc! { "id": id });
        match task.run() {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                return false;
            }
        }
    }
    pub fn get_images(&self, ip: &str) -> Vec<Image> {
        let task = self.images.find(doc! { "uploader.ip": ip });
        let images: Vec<Image> = task
            .run()
            .unwrap()
            .filter_map(|doc| {
                doc.ok().map(|d| {
                    return d;
                })
            })
            .collect();
        return images;
    }
    pub fn add_views_image(&self, id: &str) -> bool {
        let task = self.images.update_one(doc! { "id": id }, doc! { "$inc": {"views": 1} });
        let run = task.run();
        if run.is_err() {
            return false;
        }
        run.unwrap();
        true
    }
    // Bans
    pub fn get_bans(&self) -> Vec<Ban> {
        let cursor = match self.banned_users.find(doc! {}).run() {
            Ok(cursor) => cursor,
            Err(_) => {
                return vec![];
            }
        };

        let bans: Vec<Ban> = cursor
            .filter_map(|doc| doc.ok().map(|d| Ban::from(d))) // Filter out errors and convert each document to a Ban
            .collect();

        bans
    }

    pub fn check_ip(&self, ip: &str) -> Option<Ban> {
        let task = self.banned_users.find_one(doc! { "ip": ip });
        task.run().unwrap_or_default()
    }
    pub fn ban_ip(&self, ip: &str, reason: &str) -> (bool, &str) {
        if ip.len() == 0 {
            return (false, "No ip provided");
        }
        let found = self.check_ip(ip);
        if found.is_some() {
            return (false, "Ip already banned!");
        }
        let mut ban = Ban::new();
        ban.ip = ip.to_string();
        ban.reason = reason.to_string();
        ban.time = get_timestamp();

        let images: Vec<Image> = self.get_images(&ip);
        for image in images {
            self.delete_image(&image.id);
        }

        match self.banned_users.insert_one(ban).run() {
            Ok(_) => {
                return (true, "");
            }
            Err(_) => {
                return (false, "Database error!");
            }
        }
    }
    pub fn unban_ip(&self, ip: &str) -> bool {
        if self.check_ip(ip).is_none() {
            return false;
        }
        let task = self.banned_users.delete_one(doc! { "ip":ip });
        match task.run() {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                return false;
            }
        }
    }
}
