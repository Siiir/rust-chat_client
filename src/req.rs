pub type ReqClient = reqwest::blocking::Client;

pub use post_msg::{PostMsg, post_msg};
pub mod post_msg;

pub use get_msgs::{GetMsgs, get_msgs};
pub mod get_msgs;

pub mod ctxfull;

use anyhow::{anyhow, Context};
use serde::de::DeserializeOwned;
use std::io::Read;

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
