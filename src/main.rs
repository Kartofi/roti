use std::{ fs, path::Path };

use database::Database;
use dotenv::dotenv;

use choki::{ src::{ request::Request, response::Response, structs::Url }, Server };

pub mod database;
pub mod routes;
pub mod utils;
pub mod structs;

fn main() {
    dotenv().ok();
    if !Path::new("./data").exists() {
        fs::create_dir("./data").unwrap();
    }
    let mut database: Database = Database::new();

    let mut server: Server<Database> = Server::new(Some(5_000_000), Some(database));

    server.use_middleware(routes::middleware::handle);

    server.get("/admin", routes::admin::handle).unwrap();
    server.post("/admin/getbans", routes::admin::handle_get_bans).unwrap();
    server.delete("/admin/unban", routes::admin::handle_unban).unwrap();
    server.post("/admin/ban", routes::admin::handle_ban).unwrap();

    server.get("/", routes::index::handle).unwrap();
    server.get("/image/[id]", routes::image::handle).unwrap();
    server.get("/[id]", routes::view::handle).unwrap();
    server.post("/upload", routes::upload::handle).unwrap();

    server.new_static("/static", "./ui/static").unwrap();
    server.listen(3000, None, None, || { println!("🍅Roti started on port 3000!") }).unwrap();
    Server::<u8>::lock();
}
