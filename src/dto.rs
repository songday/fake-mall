use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CatelogRequest {
    pub tenant_id: String,
}

#[derive(Serialize, Debug)]
pub struct BaseResponse<'a, T: 'a + Serialize> {
    pub status: u8,
    pub data: &'a T,
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub image_url: String,
    pub price: String,
}