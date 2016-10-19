
#[macro_use(bson, doc)] extern crate bson;
extern crate mongodb;
extern crate iron_with_db;

use iron_with_db::*;
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn route_handler(req: &mut Request) -> IronResult<Response> {
    let arc = req.get::<Read<AppConfigKey>>().unwrap();
    let app = arc.as_ref();

    app.display();

    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

struct DbConfig {

    /// name of the database in the server
    dbname: String,

    /// hostname 
    hostname: String,

    /// port
    port: u16,
}

impl DbConfig {

    fn new(database_url: &str) -> DbConfig {
    
        // get the dbname
        let split = database_url.split("/").collect::<Vec<&str>>();
        let dbname = split[1].to_owned();
        
        // get the hostname
        let split = split[0].split(":").collect::<Vec<&str>>();
        let hostname = split[0].to_owned();

        // get the port
        let port: u16 = split[1].to_owned().parse().ok().expect("DbConfig: Wanted a number");

        DbConfig {
            dbname: dbname,
            hostname: hostname,
            port: port,
        }
    }

    /// obtain connection to the db
    fn get_conn(&self) -> Client {

        Client::connect(&self.hostname, self.port)
            .ok().expect("Failed to initialize mongo client")
    }
}

#[derive(Copy, Clone)]
struct DbConf;
impl Key for DbConf {
    type Value = DbConfig;
}

fn main() {

    let app = AppConfig::default();
    let dbConf = DbConfig::new(&app.database_url);

    let mut router = Router::new();
    router.get("/", route_handler, "index");
    router.get("/:query", route_handler, "query");


    let mut config = Chain::new(router);
    config.link(Read::<AppConfigKey>::both(app));
    config.link(Read::<DbConf>::both(dbConf));

    Iron::new(config).http("localhost:3000").unwrap();
}
