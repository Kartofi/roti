use std::{ fs::{ self, File }, io::Read };

use lazy_static::lazy_static;

use choki::src::{
    request::Request,
    response::Response,
    structs::{ ContentType, Header, HttpServerError, ResponseCode },
};
use crate::{ utils::{ random_num, send_file }, Database };

lazy_static! {
    static ref IMAGES_PATHS: Vec<String> = get_images_paths();
}
fn get_images_paths() -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    let paths = fs::read_dir("./ui/images").unwrap();
    for path in paths {
        res.push(path.unwrap().path().to_str().unwrap_or_default().to_string());
    }

    res
}
pub fn handle(
    req: Request,
    mut res: Response,
    database: Option<Database>
) -> Result<(), HttpServerError> {
    let random_index = random_num(0, (IMAGES_PATHS.len() - 1) as i64);

    send_file(&IMAGES_PATHS[random_index as usize], ContentType::None, &mut res)
}
