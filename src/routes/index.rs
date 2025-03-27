use std::{ fs::File, io::Read };

use choki::src::{ request::Request, response::Response, structs::ContentType };
use crate::Database;

pub fn handle(req: Request, mut res: Response, database: Option<Database>) {
    res.use_compression = true;

    crate::utils::send_file("./ui/index.html", ContentType::Html, &mut res);
}
