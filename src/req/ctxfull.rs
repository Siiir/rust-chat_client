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
