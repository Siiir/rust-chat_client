use std::{io, thread, time::Duration};

use anyhow::Context;

use super::FPATH_TO_FUTURE_MSG_ID;

/// # Warning
/// One of error resilience mechanism performs `std::thread::sleep` for 1 sec.
pub fn future_msg_id() -> crate::MsgId {
    use std::str::FromStr;

    let mut is_second_attempt = false;
    loop {
        let future_msg_id = fs_err::read_to_string(FPATH_TO_FUTURE_MSG_ID).map(|s| {
            crate::MsgId::from_str(&s).with_context(|| {
                format!(
                    "Can't parse file content as Rust's `{}`.",
                    std::any::type_name::<crate::MsgId>()
                )
            })
        });
        match future_msg_id {
            Ok(Ok(future_msg_id)) => break future_msg_id,
            Ok(anyhow::Result::Err(parse_err)) => {
                tracing::warn!("Overwriting corrupted file {FPATH_TO_FUTURE_MSG_ID:?} with defaults, due to error during content parsing: {parse_err}");
                if let Err(io_err) = crate::pa::write::default_future_msg_id_throwing_ctx_err()
                {
                    panic!("Failed to fix (overwrite) corrupted storage. Error: {io_err}")
                };
                is_second_attempt = true;
            }
            io::Result::Err(io_err) => {
                if is_second_attempt {
                    panic!("Failed to read full integer at 2nd attempt. Error: {io_err}")
                }
                is_second_attempt = true;
                if io_err.kind() == io::ErrorKind::NotFound {
                    if let Err(err) = crate::pa::init_future_msg_id() {
                        panic!("{err:?}");
                    };
                } else {
                    tracing::warn!("Can't read full integer from {FPATH_TO_FUTURE_MSG_ID:?}, reattempting after 1 sec. Error: {io_err}");
                    thread::sleep(Duration::from_secs(1));
                };
            }
        }
    }
}
