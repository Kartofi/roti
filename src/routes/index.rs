use std::{ fs::File, io::Read };

use choki::src::{ request::Request, response::Response, structs::ContentType };
use crate::Database;

pub fn handle(req: Request, mut res: Response, database: Option<Database>) {
    let mut file = File::open("./ui/index.html").unwrap();
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).unwrap();

    res.use_compression = true;
    res.send_bytes_chunked(&content, Some(ContentType::Html));
}
