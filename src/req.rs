pub type ReqClient = reqwest::blocking::Client;

pub use post_msg::PostMsg;
pub mod post_msg;

pub use get_msgs::GetMsgs;
pub mod get_msgs;

pub mod ctxfull {
    use anyhow::Context;

    pub fn get_msgs(
        client: &reqwest::blocking::Client,
        get_msgs_query: &crate::req::GetMsgs,
    ) -> anyhow::Result<Vec<crate::model::ChatMsg>> {
        super::get_msgs(client, get_msgs_query)
            .with_context(|| "Failed to get chat messages.")
    }
    pub fn post_msg(
        client: &reqwest::blocking::Client,
        post_msg_query: &crate::req::PostMsg,
    ) -> anyhow::Result<crate::model::ChatMsg> {
        super::post_msg(client, post_msg_query).with_context(|| {
            "Failed to post user-provided chat message."
        })
    }
}

use anyhow::{anyhow, Context};
use serde::de::DeserializeOwned;
use std::io::Read;

pub fn get_msgs(
    client: &reqwest::blocking::Client,
    get_msgs_query: &crate::req::GetMsgs,
) -> anyhow::Result<Vec<crate::model::ChatMsg>> {
    let resp = client
        .get("http://localhost:8080/v1/msg")
        .query(get_msgs_query)
        .send();
    interpret_resp(resp)
}
pub fn post_msg(
    client: &reqwest::blocking::Client,
    post_msg_query: &crate::req::PostMsg,
) -> anyhow::Result<crate::model::ChatMsg> {
    let resp = client
        .post("http://localhost:8080/v1/msg")
        .query(post_msg_query)
        .send();
    interpret_resp(resp)
}

fn interpret_resp<D>(
    resp: Result<reqwest::blocking::Response, reqwest::Error>,
) -> Result<D, anyhow::Error>
where
    D: DeserializeOwned,
{
    let mut resp =
        resp.context("Failed to connect with the server.")?;
    if resp.status().is_success() {
        match resp.json::<D>() {
            Ok(obj) => Ok(obj),
            Err(err) => Err(anyhow!(
                "Failed to deserialize server's response: {err}"
            )),
        }
    } else {
        let mut resp_body = vec![0; 1024];
        let read_count = resp.read(&mut resp_body).unwrap_or_else(|err| {
            tracing::warn!("Error caught while reading request body was silenced: {err}");
            0
        });
        resp_body.truncate(read_count);
        let resp_body = String::from_utf8_lossy(&resp_body);
        Err(anyhow!(
            "Server returned:\n{resp:?}\n\n{}...",
            resp_body
        ))
    }
}
