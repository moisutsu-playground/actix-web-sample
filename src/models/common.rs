use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HelloWorld {
    pub hello: String,
    pub name: String,
    pub id: u32,
}

#[derive(Deserialize)]
pub struct Add {
    pub a: i32,
    pub b: i32,
}
