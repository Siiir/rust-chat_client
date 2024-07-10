use serde::Serialize;

#[derive(derive_more::Constructor, Debug, Serialize)]
pub struct PostMsg {
    author: String,
    content: String,
}
