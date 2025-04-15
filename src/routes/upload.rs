use std::{ env, fs::{ self, File }, io::{ Read, Write } };

use choki::src::{
    request::Request,
    response::Response,
    structs::{ ContentType, HttpServerError, ResponseCode },
};
use crate::{ structs::Image, utils::{ self, get_timestamp, redirect }, Database, DATA_PATH };

pub fn handle(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let database = database.unwrap();

    let body = req.body();
    if req.content_type.clone().unwrap() != ContentType::MultipartForm {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_string("Only MultipartForm allowed!");
    }
    if body.len() != 1 {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_string("Only one file allowed!");
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
        return res.send_string("No file!");
    }
    let allowed_extension = utils::is_extension_allowed(&image.file_name);

    if allowed_extension.0 == false {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_string("Only images allowed!");
    }
    let file_path = DATA_PATH.to_string() + &id + allowed_extension.1;

    image.file_path = file_path.clone();
    image.file_type = allowed_extension.2.as_str().to_string();

    if database.add_image(image) {
        let mut file = File::create(file_path).unwrap();
        file.write_all(body_item.data).unwrap();
    } else {
        res.set_status(&ResponseCode::BadRequest);
        return res.send_string("Server ERROR!");
    }

    redirect(&mut res, &("/".to_string() + &id))
}
