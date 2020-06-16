use crate::handler;
use tide::{self, Server};

pub async fn start() -> std::io::Result<()> {
    let mut app = tide::new();
    setup_route(&mut app);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn setup_route(app: &mut Server<()>) {
    app.at("/").get(handler::index);
    app.at("/categories").get(handler::get_categories);
    app.at("/items").get(handler::get_items);
    app.at("/s").get(handler::scrape);
    app.at("/hello").get(|_| async move { Ok("Hello, world!") });
}
