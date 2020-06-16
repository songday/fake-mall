use tide::{Request, Response, StatusCode, Result};

use crate::dto::{CatelogRequest};
use crate::scraper;

pub async fn index(_req: Request<()>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(String::from("This is a fake mall"));
    Ok(res)
}

pub async fn get_categories(req: Request<()>) -> Result {
    // 下面两行
    // let p : CatelogRequest = req.body_json().await.unwrap();
    let p = match req.query::<CatelogRequest>() {
        Ok(r) => r,
        Err(e) => {
            println!("{:?}", e);
            CatelogRequest {
                tenant_id: "".into()
            }
        }
    };
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(format!("{:?}", p));
    Ok(res)
}

pub async fn get_items(mut req: Request<()>) -> Result {
    Ok(Response::new(StatusCode::Ok))
}

pub async fn scrape(req: Request<()>) -> Result {
    scraper::get_items("http://al.tourting.com/shop-3.html").await;
    Ok(Response::new(StatusCode::Ok))
}