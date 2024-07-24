use std::io;

use anyhow::Context;

use crate::pa::{DEAFULT_FUTURE_MSG_ID, FPATH_TO_FUTURE_MSG_ID};

pub fn default_future_msg_id_throwing_ctx_err() -> anyhow::Result<()> {
    future_msg_id_throwing_ctx_err(DEAFULT_FUTURE_MSG_ID)
}
pub fn future_msg_id_throwing_ctx_err(new_val: crate::MsgId) -> anyhow::Result<()> {
    future_msg_id(new_val).context("Failed to overwrite `future message id` with {new_val}.")
}
#[allow(dead_code)] // future use
fn default_future_msg_id() -> io::Result<()> {
    future_msg_id(DEAFULT_FUTURE_MSG_ID)
}
fn future_msg_id(new_val: crate::MsgId) -> io::Result<()> {
    fs_err::write(FPATH_TO_FUTURE_MSG_ID, new_val.to_string())
}
