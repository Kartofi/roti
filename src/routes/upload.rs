use std::{ env, fs::{ self, File }, io::{ Read, Write } };

use choki::src::{ request::Request, response::Response, structs::{ ContentType, ResponseCode } };
use crate::{ structs::Image, utils::{ self, get_timestamp }, Database };

pub fn handle(req: Request, mut res: Response, database: Option<Database>) {
    let database = database.unwrap();
    let ban = database.check_ip(&req.ip.clone().unwrap_or_default());
    if ban.is_some() {
        let ban = ban.unwrap();
        res.set_status(&ResponseCode::BadRequest);
        res.send_string(&format!("Banned from uploading! Reason: {}", ban.reason));
        return;
    }

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

    let mut image = Image::new();
    let id = image.id.clone();
    //Uploader
    image.uploader.ip = req.ip.clone().unwrap_or_default();
    image.uploader.user_agent = req.user_agent.clone().unwrap_or_default();
    //
    image.file_name = body_item.info.file_name.clone().unwrap_or_default();
    image.size = body_item.data.len() as u64;
    image.upload_time = get_timestamp();

    if image.file_name.is_empty() {
        res.set_status(&ResponseCode::BadRequest);
        res.send_string("No file!");
        return;
    }
    let allowed_extension = utils::is_extension_allowed(&image.file_name);

    if allowed_extension.0 == false {
        res.set_status(&ResponseCode::BadRequest);
        res.send_string("Only images allowed!");
        return;
    }
    let file_path = env::var("DATA").unwrap() + &id + allowed_extension.1;

    image.file_path = file_path.clone();
    image.file_type = allowed_extension.2.as_str().to_string();

    if database.add_image(image) {
        let mut file = File::create(file_path).unwrap();
        file.write_all(body_item.data).unwrap();
    } else {
        res.set_status(&ResponseCode::BadRequest);
        res.send_string("Server ERROR!");
        return;
    }

    let mut file = File::open("./ui/uploaded.html").unwrap();
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).unwrap();
    let mut string_content = String::from_utf8_lossy(&content).to_string();
    string_content = string_content.replace("[IMAGEURL]", &("/".to_string() + &id));

    res.use_compression = true;
    res.send_bytes_chunked(&string_content.as_bytes(), Some(ContentType::Html));
}
