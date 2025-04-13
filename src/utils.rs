use std::{ fs::File, io::Read, time::{ SystemTime, UNIX_EPOCH } };
use choki::src::{ response::Response, structs::{ ContentType, HttpServerError } };
use rand::Rng;

const CHARSET: &str = "abcdefghijklmnopqrstuvwxzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const ALLOWED_EXTENSIONS: [(&str, ContentType); 5] = [
    (".gif", ContentType::Gif),
    (".jpg", ContentType::Jpeg),
    (".jpeg", ContentType::Jpeg),
    (".png", ContentType::Png),
    (".webp", ContentType::Webp),
];
const ID_LENGTH: u64 = 20;

pub fn get_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
pub fn random_num(min: i64, max: i64) -> i64 {
    let mut rng = rand::rng();
    if min == max {
        return min;
    }
    rng.random_range(min..max + 1)
}

pub fn random_char() -> char {
    let mut rng = rand::rng();
    CHARSET.as_bytes()[rng.random_range(0..=CHARSET.len() - 1)] as char
}
pub fn get_id(length: u64) -> String {
    let mut output = String::new();
    for i in 0..length {
        output.push(random_char());
    }

    output
}
pub fn get_id_default() -> String {
    get_id(ID_LENGTH)
}

pub fn is_extension_allowed(input: &str) -> (bool, &str, ContentType) {
    let lower_input = input.to_lowercase();

    for extension in ALLOWED_EXTENSIONS {
        if lower_input.ends_with(&extension.0.to_lowercase()) {
            return (true, extension.0, extension.1);
        }
    }
    return (false, "", ContentType::None);
}
// Http stuff

pub fn send_file(
    path: &str,
    content_type: ContentType,
    res: &mut Response
) -> Result<(), HttpServerError> {
    let mut file = File::open(path).unwrap();
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).unwrap();

    res.send_bytes_chunked(&content, Some(content_type))
}
