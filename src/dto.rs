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