use std::{ env, fs::File, io::Read };

use choki::src::{ request::Request, response::Response, structs::{ ContentType, ResponseCode } };
use crate::{ database, structs::Ban, Database };

pub fn handle(req: Request, mut res: Response, database: Option<Database>) {
    res.use_compression = true;

    crate::utils::send_file("./ui/admin.html", ContentType::Html, &mut res);
}

pub fn handle_ban(req: Request, mut res: Response, database: Option<Database>) {
    if !handle_password_check(&req, &mut res) {
        return;
    }

    res.use_compression = true;
    let database = database.unwrap();
    database.ban_ip(&req.ip.unwrap_or_default(), "NOOO");
    crate::utils::send_file("./ui/index.html", ContentType::Html, &mut res);
}
pub fn handle_unban(req: Request, mut res: Response, database: Option<Database>) {
    if !handle_password_check(&req, &mut res) {
        return;
    }

    res.use_compression = true;
    let database = database.unwrap();
    database.ban_ip(&req.ip.unwrap_or_default(), "NOOO");
    crate::utils::send_file("./ui/index.html", ContentType::Html, &mut res);
}
pub fn handle_get_bans(req: Request, mut res: Response, database: Option<Database>) {
    if !handle_password_check(&req, &mut res) {
        return;
    }

    res.use_compression = true;
    let database = database.unwrap();
    let bans: Vec<Ban> = database.get_bans();

    let bson = serde_json::to_string(&bans).unwrap_or_default();

    res.send_json(&bson);
}
fn handle_password_check(req: &Request, res: &mut Response) -> bool {
    let body = req.body();
    if body.len() != 1 || body[0].info.clone().name.unwrap_or_default() != "password" {
        res.set_status(&ResponseCode::BadRequest);
        res.send_string("No Password!");
        return false;
    }
    let valid = check_password(&String::from_utf8_lossy(&body[0].data));
    if !valid {
        res.set_status(&ResponseCode::BadRequest);
        res.send_string("Wrong password!");
        return false;
    }
    return true;
}
fn check_password(password: &str) -> bool {
    return password == env::var("ADMIN_PASSWORD").unwrap();
}
