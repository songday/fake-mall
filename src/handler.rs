use crate::dto::{CatelogRequest};

use tide::{Request, Response};

pub async fn index(_req: Request<()>) -> Response {
    Response::new(200).body_string(String::from("This is a fake mall"))
}

pub async fn get_categories(req: Request<()>) -> Response {
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
    Response::new(200).body_string(format!("{:?}", p))
}

pub async fn get_items(mut req: Request<()>) -> Response {}