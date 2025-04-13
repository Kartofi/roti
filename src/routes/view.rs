use std::{ fs::File, io::Read };

use choki::src::{
    request::Request,
    response::Response,
    structs::{ ContentType, Header, HttpServerError, ResponseCode },
};
use crate::Database;

pub fn handle(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let id = req.params.get("id").unwrap();

    let database = database.unwrap();
    let result = database.get_image(id);
    if result.is_none() {
        res.set_status(&ResponseCode::NotFound);
        return res.send_string("Invalid id!");
    }
    let result = result.unwrap();

    let mut file = File::open("./ui/view.html").unwrap();
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).unwrap();

    let mut string_content = String::from_utf8_lossy(&content).to_string();

    string_content = string_content.replace("[IMAGEURL]", &("/image/".to_string() + &id));
    string_content = string_content.replace("[IMAGESIZE]", &result.size.to_string());
    string_content = string_content.replace("[FILENAME]", &result.file_name);
    string_content = string_content.replace("[VIEWS]", &result.views.to_string());

    res.use_compression = true;
    res.send_bytes_chunked(&string_content.as_bytes(), Some(ContentType::Html))
}
