use database::Database;
use dotenv::dotenv;

use choki::Server;

pub mod database;
pub mod routes;
pub mod utils;
pub mod structs;

fn main() {
    dotenv().ok();

    let mut database: Database = Database::new();

    let mut server: Server<Database> = Server::new(Some(0), Some(database));

    server.get("/", routes::index::handle).unwrap();
    server.post("/upload", routes::upload::handle).unwrap();

    server.new_static("/static", "./ui/static").unwrap();
    server.listen(3000, None, Some(100), || {}).unwrap();
    Server::<u8>::lock();
}
