use std::{ fs::File, io::Read };

use choki::src::{
    request::Request,
    response::Response,
    structs::{ ContentType, Header, ResponseCode },
};
use crate::Database;

pub fn handle(req: Request, mut res: Response, database: Option<Database>) {
    let id = req.params.get("id").unwrap();

    let database = database.unwrap();
    let result = database.get_image(id);
    if result.is_none() {
        let mut file = File::open("./ui/static/not_found.png").unwrap();
        let mut content: Vec<u8> = Vec::new();
        file.read_to_end(&mut content).unwrap();

        res.send_bytes_chunked(&content, Some(ContentType::Png));
        return;
    }
    let result = result.unwrap();

    let mut file = File::open(result.file_path).unwrap();
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).unwrap();

    res.send_bytes_chunked(&content, Some(ContentType::from_string(&result.file_type)));
}
