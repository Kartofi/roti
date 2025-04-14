use std::{ env, fs::File, io::Read };

use bson::doc;
use choki::src::{
    request::Request,
    response::Response,
    structs::{ BodyItem, ContentType, Cookie, HttpServerError, ResponseCode },
};
use crate::{
    database,
    structs::Ban,
    utils::get_timestamp,
    Database,
    ADMIN_PASSWORD,
    SESSION_EXPIRE_TIME,
};

pub fn handle(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    res.use_compression = true;

    crate::utils::send_file("./ui/admin.html", ContentType::Html, &mut res)
}

pub fn handle_login(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    res.use_compression = true;

    crate::utils::send_file("./ui/admin.html", ContentType::Html, &mut res)
}

// Controls
pub fn handle_ban(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let database = database.unwrap();

    let body = req.body();
    let ip = req.ip.clone().unwrap_or_default();

    if !handle_session_check(&ip, &req.cookies, &database) {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"Session not matching!" }).to_string()
        );
    }

    if body.len() != 2 {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"No ip or reason provided!" }).to_string()
        );
    }

    let result = database.ban_ip(
        &String::from_utf8_lossy(body[0].data),
        &String::from_utf8_lossy(body[1].data)
    );

    res.use_compression = true;
    res.send_json(&(doc! { "result":result.0,"error":result.1 }).to_string())
}
pub fn handle_unban(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let database = database.unwrap();

    let body = req.body();
    let ip = req.ip.clone().unwrap_or_default();

    if !handle_session_check(&ip, &req.cookies, &database) {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"Session not matching!" }).to_string()
        );
    }
    if body.len() != 1 {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(&(doc! { "result":false,"error":"No ip provided!" }).to_string());
    }

    let result = database.unban_ip(&String::from_utf8_lossy(body[0].data));

    res.use_compression = true;
    res.send_json(&(doc! { "result":result,"error":"" }).to_string())
}
pub fn handle_get_bans(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let database = database.unwrap();

    let ip = req.ip.clone().unwrap_or_default();

    if !handle_session_check(&ip, &req.cookies, &database) {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"Session not matching!" }).to_string()
        );
    }

    let bans: Vec<Ban> = database.get_bans();

    let bson = serde_json::to_string(&bans).unwrap_or_default();

    res.use_compression = true;
    res.send_json(&bson)
}
fn handle_session_check(ip: &str, cookies: &Vec<Cookie>, database: &Database) -> bool {
    let session_cookie = cookies.iter().find(|item| item.name == "Session");
    if session_cookie.is_none() {
        return false;
    }
    let session_cookie = session_cookie.unwrap();

    let session = database.get_session(&session_cookie.value);
    if session.is_none() {
        return false;
    }
    let session = session.unwrap();
    if session.ip != ip {
        return false;
    }

    if get_timestamp() - session.expire_time <= SESSION_EXPIRE_TIME {
        database.remove_session(&session.id);
        return false;
    }

    return true;
}
fn check_password(password: &str) -> bool {
    return password == ADMIN_PASSWORD.as_str();
}
