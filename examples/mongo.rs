
#[macro_use(bson, doc)] extern crate bson;
extern crate mongodb;
extern crate iron_with_db;

use iron_with_db::prelude::*;
use iron_with_db::db::{DatabaseConfig, DatabaseConfigKey};
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

// MongoClientKey struct for passing mongo client into the chain
#[derive(Copy, Clone)]
struct MongoClientKey;

impl Key for MongoClientKey {
    type Value = Client;
}

// Generic Route Handler
fn route_handler(req: &mut Request) -> IronResult<Response> {

    // reference to url query
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    
    // querystr for multiple uses
    let querystr: String = String::from(*query);

    // configuration for db and db connection taken from persistent state objs
    let db_conf = req.extensions.get::<Read<DatabaseConfigKey>>().unwrap();
    let db_conn = req.extensions.get::<Read<MongoClientKey>>().unwrap();

    // grabbing collection
    let coll = db_conn.db(&db_conf.dbname).collection("hits");
    
    // will be used as search parameters in the db search
    let hit_counter_search = doc! {
        "query" => querystr
    };

    // get the result from the db query
    let mut cursor = coll.find(None, None).ok().expect("Couldn't Find");

    let item = cursor.next();

    match item {
        Some(Ok(x)) => println!("{:?}", x.get("title")),
        Some(Err(_)) => panic!("Error getting data"),
        None => println!("None!"),
    }
    
    //  db_conf.display();

    Ok(Response::with((status::Ok, *query)))
}


fn main() {

    let db_conf = DatabaseConfig::new("localhost:27017/test");
    let db_conn = Client::connect(&db_conf.hostname, db_conf.port)
        .ok().expect("Failed to initialize db connection");

    let mut router = Router::new();
    router.get("/", route_handler, "index");
    router.get("/:query", route_handler, "query");

    let mut config = Chain::new(router);
    config.link(Read::<DatabaseConfigKey>::both(db_conf));
    config.link(Read::<MongoClientKey>::both(db_conn));

    println!("Running app on localhost:3000");
    Iron::new(config).http("localhost:3000").unwrap();
}
