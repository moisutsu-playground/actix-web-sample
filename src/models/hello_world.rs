use serde::Serialize;

#[derive(Serialize)]
pub struct HelloWorld {
    pub hello: String,
    pub name: String,
    pub id: u32,
}
