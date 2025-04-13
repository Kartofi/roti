use std::{ fs::File, io::Read };

use choki::src::{
    request::Request,
    response::Response,
    structs::{ ContentType, Header, HttpServerError, ResponseCode },
    utils,
};
use crate::Database;

pub fn handle(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let id = req.params.get("id").map_or("", |v| v);
    if id.len() == 0 {
        return crate::utils::send_file("./ui/static/not_found.png", ContentType::Png, &mut res);
    }
    let database = database.unwrap();
    let result = database.get_image(id);
    if result.is_none() {
        return crate::utils::send_file("./ui/static/not_found.png", ContentType::Png, &mut res);
    }
    let result = result.unwrap();

    database.add_views_image(id);
    crate::utils::send_file(
        &result.file_path,
        ContentType::from_string(&result.file_type).unwrap_or(ContentType::None),
        &mut res
    )
}
