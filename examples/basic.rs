
#[macro_use(bson, doc)] extern crate bson;
extern crate mongodb;
extern crate iron_with_db;

use iron_with_db::prelude::*;
use iron_with_db::db::{DatabaseConfig, DatabaseConfigKey};

// Generic Route Handler: puts up DbConf to html response, 
// something you should never actually do ;)
fn route_handler(req: &mut Request) -> IronResult<Response> {
    
    let db_conf = req.extensions.get::<Read<DatabaseConfigKey>>().unwrap();
    Ok(Response::with((status::Ok, db_conf.display())))
}


fn main() {

    let db_conf = DatabaseConfig::new("localhost:27017/test");

    let mut router = Router::new();
    router.get("/", route_handler, "index");
    router.get("/:query", route_handler, "query");

    let mut config = Chain::new(router);
    config.link(Read::<DatabaseConfigKey>::both(db_conf));

    println!("Running app on localhost:3000");
    Iron::new(config).http("localhost:3000").unwrap();
}
