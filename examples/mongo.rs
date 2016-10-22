
#[macro_use(bson, doc)] extern crate bson;
extern crate mongodb;
extern crate iron_with_db;

use iron_with_db::*;
use iron_with_db::db::{DatabaseConfig, DatabaseConfigKey};
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn route_handler(req: &mut Request) -> IronResult<Response> {
    let arc = req.get::<Read<DatabaseConfigKey>>().unwrap();
    let dbConf = arc.as_ref();

    dbConf.display();

    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn main() {

    let dbConf = DatabaseConfig::new("localhost:27017/test");

    let mut router = Router::new();
    router.get("/", route_handler, "index");
    router.get("/:query", route_handler, "query");

    let mut config = Chain::new(router);
    config.link(Read::<DatabaseConfigKey>::both(dbConf));

    Iron::new(config).http("localhost:3000").unwrap();
}
