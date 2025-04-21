use std::{ fs::File, io::Read };

use bson::Document;
use choki::src::{
    request::Request,
    response::Response,
    structs::{ ContentType, Header, HttpServerError, ResponseCode },
};
use crate::{ structs::User, utils::send_file, Database };

pub fn handle(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    res.use_compression = true;

    send_file("./ui/uploads.html", ContentType::Html, &mut res)
}
pub fn handle_fetch(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let ip = req.ip.unwrap_or_default();

    let database = database.unwrap();

    let mut result = database.get_images(&ip);

    let bsons: Vec<Document> = result
        .iter()
        .map(|item| item.to_bson_simple())
        .collect();

    let bson = bson::to_bson(&bsons).unwrap_or_default();

    res.use_compression = true;
    res.send_json_chunked(&bson.to_string())
}
