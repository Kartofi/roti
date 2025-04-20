use std::{ env, fs::File, io::Read };

use bson::doc;
use choki::src::{
    request::Request,
    response::Response,
    structs::{ BodyItem, ContentType, Cookie, Header, HttpServerError, ResponseCode },
};
use chrono::{ TimeZone, Utc };
use crate::{
    database,
    structs::{ Ban, Session },
    utils::{ get_id, get_timestamp, redirect },
    Database,
    ADMIN_PASSWORD,
    SESSION_EXPIRE_TIME,
    SESSION_ID_LENGTH,
};
// GET /admin
pub fn handle(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    res.use_compression = true;

    let database = database.unwrap();

    let ip = req.ip.clone().unwrap_or_default();

    if !handle_session_check(&ip, &req.cookies, &database) {
        return crate::utils::send_file("./ui/login.html", ContentType::Html, &mut res);
    }

    crate::utils::send_file("./ui/admin.html", ContentType::Html, &mut res)
}
// POST /admin/login
pub fn handle_login(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    res.use_compression = true;

    let body = req.body();

    if !is_multipartform(&req.content_type) || body.len() != 1 {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"Body must be multipart/form" }).to_string()
        );
    }

    let database = database.unwrap();

    let ip = req.ip.clone().unwrap_or_default();

    if handle_session_check(&ip, &req.cookies, &database) {
        return crate::utils::send_file("./ui/login.html", ContentType::Html, &mut res);
    }

    let password = &body[0];

    if check_password(&String::from_utf8_lossy(password.data)) {
        let mut session = Session::new();
        session.id = get_id(SESSION_ID_LENGTH);
        session.expire_time = get_timestamp() + SESSION_EXPIRE_TIME;
        session.ip = ip;

        let result = database.add_session(&session);

        if result {
            let mut cookie = Cookie::new_simple("roti_session".to_string(), session.id);

            cookie.expires = Utc.timestamp_opt(session.expire_time as i64, 0)
                .unwrap()
                .format("%a, %d %b %Y %H:%M:%S GMT")
                .to_string();

            res.set_cookie(&cookie);
        }
    }

    redirect(&mut res, "/admin")
}

// Controls
// POST /admin/ban
pub fn handle_ban(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    res.use_compression = true;
    if !is_multipartform(&req.content_type) {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"Body must be multipart/form" }).to_string()
        );
    }

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

    res.send_json(&(doc! { "result":result.0,"error":result.1 }).to_string())
}
// POST /admin/unban
pub fn handle_unban(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    res.use_compression = true;
    if !is_multipartform(&req.content_type) {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"Body must be multipart/form" }).to_string()
        );
    }

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

    res.send_json(&(doc! { "result":result,"error":"" }).to_string())
}
// POST /admin/getbans
pub fn handle_get_bans(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    res.use_compression = true;

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

    res.send_json(&bson)
}

// Utils
fn handle_session_check(ip: &str, cookies: &Vec<Cookie>, database: &Database) -> bool {
    let session_cookie = cookies.iter().find(|item| item.name == "roti_session");
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

    if get_timestamp() - session.expire_time >= SESSION_EXPIRE_TIME {
        database.remove_session(&session.id);
        return false;
    }

    return true;
}

fn check_password(password: &str) -> bool {
    return password == ADMIN_PASSWORD.as_str();
}

fn is_multipartform(input: &Option<ContentType>) -> bool {
    match input {
        Some(content_type) => {
            if content_type == &ContentType::MultipartForm {
                return true;
            }
            return false;
        }
        None => {
            return false;
        }
    }
}
