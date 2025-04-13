use std::{ fs::File, io::Read };

use choki::src::{ request::Request, response::Response, structs::{ ContentType, HttpServerError } };
use crate::{ database, Database };

pub fn handle(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let database = database.unwrap();

    let total_bans = database.total_bans();
    let total_size = database.total_size();
    let total_views = database.total_views();

    let mut file = File::open("./ui/index.html").unwrap();
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).unwrap();

    let mut string_content = String::from_utf8_lossy(&content).to_string();

    string_content = string_content.replace("[TOTAL_BANS]", &total_bans.to_string());
    string_content = string_content.replace("[TOTAL_SIZE]", &total_size.to_string());
    string_content = string_content.replace("[TOTAL_VIEWS]", &total_views.to_string());

    res.use_compression = true;
    res.send_bytes_chunked(&string_content.as_bytes(), Some(ContentType::Html))
}
