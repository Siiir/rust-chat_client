//! # PA – Persistance API

pub mod read;
pub mod write;

use std::io::{self, Write};

use anyhow::Context;

pub const FPATH_TO_FUTURE_MSG_ID: &str = "./state/next_msg_id.text";
pub const DEAFULT_FUTURE_MSG_ID: crate::MsgId = 0;

pub fn init_future_msg_id() -> anyhow::Result<()> {
    create_new_future_msg_id()
        .context("Failed to save initial value of `future message id` in persistent storage.")
}
fn create_new_future_msg_id() -> io::Result<()> {
    if let Some(parent) = std::path::Path::new(FPATH_TO_FUTURE_MSG_ID).parent() {
        fs_err::create_dir_all(parent)?;
    }
    fs_err::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(FPATH_TO_FUTURE_MSG_ID)?
        .write_all(DEAFULT_FUTURE_MSG_ID.to_string().as_bytes())
}
