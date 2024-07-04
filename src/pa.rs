//! PA – Persistance API

use std::io::{self, Write};

use anyhow::Context;

pub const FPATH_TO_FUTURE_MSG_ID: &str = "./state/next_msg_id.text";
pub const DEAFULT_FUTURE_MSG_ID: crate::MsgId = 0;

pub mod read;

pub fn init_future_msg_id() -> anyhow::Result<()> {
    create_new_future_msg_id()
        .context("Failed to save initial value of `future message id` in persistent storage.")
}
fn create_new_future_msg_id() -> io::Result<()> {
    fs_err::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(FPATH_TO_FUTURE_MSG_ID)?
        .write_all(DEAFULT_FUTURE_MSG_ID.to_string().as_bytes())
}
pub fn write_future_msg_id_with_default_throwing_ctx_err() -> anyhow::Result<()> {
    write_future_msg_id_throwing_ctx_err(DEAFULT_FUTURE_MSG_ID)
}
pub fn write_future_msg_id_throwing_ctx_err(new_val: crate::MsgId) -> anyhow::Result<()> {
    write_future_msg_id(new_val).context("Failed to write `future message id` with {new_val}.")
}
#[allow(dead_code)] // future use
fn write_future_msg_id_with_default() -> io::Result<()> {
    write_future_msg_id(DEAFULT_FUTURE_MSG_ID)
}
fn write_future_msg_id(new_val: crate::MsgId) -> io::Result<()> {
    fs_err::write(FPATH_TO_FUTURE_MSG_ID, new_val.to_string())
}
