use std::{ env, fs, path::Path };

use database::Database;
use dotenv::dotenv;

use choki::{ src::{ request::Request, response::Response, structs::Url }, Server };
use lazy_static::lazy_static;

pub mod database;
pub mod routes;
pub mod utils;
pub mod structs;

pub static SESSION_ID_LENGTH: u64 = 50;
pub static SESSION_EXPIRE_TIME: i64 = 50;

lazy_static! {
    static ref DATA_PATH: String = get_var("DATA");
    static ref MONGODB_URI: String = get_var("MONGODB_URI");
    static ref ADMIN_PASSWORD: String = get_var("ADMIN_PASSWORD");
}
fn get_var(name: &str) -> String {
    dotenv().ok();
    return env::var(name).expect(&format!("Var {} haven`t been set.", name));
}

fn main() {
    if !Path::new(DATA_PATH.as_str()).exists() {
        fs::create_dir(DATA_PATH.as_str()).unwrap();
    }
    let mut database: Database = Database::new();

    let mut server: Server<Database> = Server::new(Some(5_000_000), Some(database));

    server.use_middleware(routes::middleware::handle);

    server.get("/admin", routes::admin::handle).unwrap();
    server.post("/login", routes::admin::handle_login).unwrap();

    server.post("/admin/getbans", routes::admin::handle_get_bans).unwrap();
    server.delete("/admin/unban", routes::admin::handle_unban).unwrap();
    server.post("/admin/ban", routes::admin::handle_ban).unwrap();

    server.get("/", routes::index::handle).unwrap();
    server.get("/image/[id]", routes::image::handle).unwrap();
    server.get("/[id]", routes::view::handle).unwrap();
    server.post("/upload", routes::upload::handle).unwrap();

    server.get("/randomimage", routes::random_image::handle).unwrap();

    server.new_static("/static", "./ui/static").unwrap();
    server.listen(3000, None, None, || { println!("🍅Roti started on port 3000!") }).unwrap();
    Server::<u8>::lock();
}
