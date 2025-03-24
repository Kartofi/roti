use std::{ fs::File, io::Read };

use choki::src::{ request::Request, response::Response, structs::{ ContentType, ResponseCode } };
use crate::{ structs::Image, utils::get_timestamp, Database };

pub fn handle(req: Request, mut res: Response, database: Option<Database>) {
    let body = req.body();
    if req.content_type.clone().unwrap() != ContentType::MultipartForm {
        res.set_status(&ResponseCode::BadRequest);
        res.send_string("Only MultipartForm allowed!");
        return;
    }
    if body.len() != 1 {
        res.set_status(&ResponseCode::BadRequest);
        res.send_string("Only one file allowed!");
        return;
    }
    let body_item = &body[0];

    let mut image = Image::new("", "", 0, 0);

    image.file_name = body_item.info.file_name.clone().unwrap_or_default();
    image.size = body_item.data.len() as u64;
    image.upload_time = get_timestamp();

    if image.file_name.len() == 0 {
        res.set_status(&ResponseCode::BadRequest);
        res.send_string("No file!");
        return;
    }
    let database = database.unwrap();
    database.add_image(image);

    let mut file = File::open("./ui/uploaded.html").unwrap();
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).unwrap();
    res.use_compression = true;
    res.send_bytes_chunked(&content, Some(ContentType::Html));
}
