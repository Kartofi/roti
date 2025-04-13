use std::{ env, fs::File, io::Read };

use bson::doc;
use choki::src::{
    request::Request,
    response::Response,
    structs::{ BodyItem, ContentType, HttpServerError, ResponseCode },
};
use crate::{ database, structs::Ban, Database, ADMIN_PASSWORD };

pub fn handle(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    res.use_compression = true;

    crate::utils::send_file("./ui/admin.html", ContentType::Html, &mut res)
}

pub fn handle_ban(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let body = req.body();
    if !handle_password_check(&body) || req.body().len() != 3 {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"Wrong or missing password!" }).to_string()
        );
    }
    if req.body().len() != 3 {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"No ip or reason provided!" }).to_string()
        );
    }

    let database = database.unwrap();
    let result = database.ban_ip(
        &String::from_utf8_lossy(body[1].data),
        &String::from_utf8_lossy(body[2].data)
    );

    res.use_compression = true;
    res.send_json(&(doc! { "result":result.0,"error":result.1 }).to_string())
}
pub fn handle_unban(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let body = req.body();
    if !handle_password_check(&body) {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"Wrong or missing password!" }).to_string()
        );
    }
    if body.len() != 2 {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"No ip or reason provided!" }).to_string()
        );
    }

    let database = database.unwrap();
    let result = database.unban_ip(&String::from_utf8_lossy(body[1].data));

    res.use_compression = true;
    res.send_json(&(doc! { "result":result,"error":"" }).to_string())
}
pub fn handle_get_bans(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let body = req.body();
    if !handle_password_check(&body) {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_json(
            &(doc! { "result":false,"error":"Wrong or missing password!" }).to_string()
        );
    }

    let database = database.unwrap();

    let bans: Vec<Ban> = database.get_bans();

    let bson = serde_json::to_string(&bans).unwrap_or_default();

    res.use_compression = true;
    res.send_json(&bson)
}
fn handle_password_check(body: &Vec<BodyItem<'_>>) -> bool {
    if body.len() == 0 || body[0].info.clone().name.unwrap_or_default() != "password" {
        return false;
    }
    let valid = check_password(&String::from_utf8_lossy(&body[0].data));
    if !valid {
        return false;
    }
    return true;
}
fn check_password(password: &str) -> bool {
    return password == ADMIN_PASSWORD.as_str();
}
