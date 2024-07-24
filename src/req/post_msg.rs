use serde::Serialize;

pub mod factory;

pub fn post_msg(
    client: &reqwest::blocking::Client,
    post_msg_query: &crate::req::PostMsg,
) -> anyhow::Result<crate::model::ChatMsg> {
    let url = format!("http://{}/v1/msg", crate::app::cfg().server_addr);
    let resp = client
        .post(url)
        .query(post_msg_query)
        .send();
    crate::req::interpret_resp(resp)
}

#[derive(derive_more::Constructor, Debug, Serialize)]
pub struct PostMsg {
    author: String,
    content: String,
}
