use std::{ fs::File, io::Read };

use choki::src::{
    request::Request,
    response::Response,
    structs::{ ContentType, Header, ResponseCode },
    utils,
};
use crate::Database;

pub fn handle(req: Request, mut res: Response, database: Option<Database>) {
    let id = req.params.get("id").unwrap();

    let database = database.unwrap();
    let result = database.get_image(id);
    if result.is_none() {
        crate::utils::send_file("./ui/static/not_found.png", ContentType::Png, &mut res);
        return;
    }
    let result = result.unwrap();

    database.add_views_image(id);
    crate::utils::send_file(
        &result.file_path,
        ContentType::from_string(&result.file_type),
        &mut res
    );
}
