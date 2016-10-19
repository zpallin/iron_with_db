extern crate iron;
extern crate router;
extern crate persistent;
extern crate iron_with_db;

use iron_with_db::*;

fn route_handler(req: &mut Request) -> IronResult<Response> {
    let arc = req.get::<Read<AppConfigKey>>().unwrap();
    let app = arc.as_ref();

    app.display();

    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}


fn main() {
 
    let mut router = Router::new();
    router.get("/", route_handler, "index");
    router.get("/:query", route_handler, "query");

    let app = AppConfig::default();

    let mut config = Chain::new(router);
    config.link(Read::<AppConfigKey>::both(app));

    Iron::new(config).http("localhost:3000").unwrap();
}
