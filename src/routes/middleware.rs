use std::{ fs::File, io::Read };

use choki::src::{
    request::Request,
    response::Response,
    structs::{ ContentType, ResponseCode, Url },
};

use crate::{ database, Database };

pub fn handle(url: &Url, req: &Request, res: &mut Response, database: &Option<Database>) -> bool {
    let ip = req.ip.clone().unwrap_or_default();

    let ban = database.clone().unwrap().check_ip(&ip);
    if ban.is_none() {
        return true;
    }
    let ban = ban.unwrap();
    res.set_status(&ResponseCode::BadRequest);
    res.send_string(&format!("Ip banned! Reason: {}", ban.reason));
    return false;
}
