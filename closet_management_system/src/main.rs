use afire::{Method, Response, ServeStatic, Server};

fn main() {
    let mut server = Server::new("localhost", 9090);

    ServeStatic::new("./data/static").attach(&mut server);

    server.route(Method::GET, "/api/data", |_req| {
        Response::new().text(r#"[{"name": "Camping Tent", "in": 3, "total": 6, "img": "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse1.mm.bing.net%2Fth%3Fid%3DOIP.kc_W7R88Da02SBPP6FQ8bwHaE8%26pid%3DApi&f=1"}, {"name": "Camping Toast", "in": 1, "total": 1, "img": "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse3.mm.bing.net%2Fth%3Fid%3DOIP.k-eule--8QINEujhWYJAygHaHa%26pid%3DApi&f=1"}, {"name": "Camping Stove", "in": 0, "total": 1, "img": "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fsc01.alicdn.com%2Fkf%2FHTB17YmPdRiE3KVjSZFMq6zQhVXaB%2F200329072%2FHTB17YmPdRiE3KVjSZFMq6zQhVXaB.jpg&f=1&nofb=1"}]"#)
    });

    server.start().unwrap();
}
