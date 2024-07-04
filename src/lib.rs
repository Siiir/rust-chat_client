pub mod model;
use std::thread;

pub use ui::cli;
pub mod init;
pub mod pa;
pub mod req;
pub mod ui;
pub mod util;

pub type MsgId = i64;

pub fn start_msg_fetching_deamon(
    mut messages: Vec<model::ChatMsg>,
    client: reqwest::blocking::Client,
) -> thread::JoinHandle<impl Send + 'static> {
    use std::time::Duration;

    thread::spawn(move || {
        let mut get_msgs_query = model::GetMsgs::default();
        loop {
            let start = chrono::Utc::now();

            // Perform query
            messages.last().map(|msg| msg.id() + 1).map(|next_msg_id| {
                fs_err::write(crate::pa::FPATH_TO_FUTURE_MSG_ID, next_msg_id.to_le_bytes())
                    .unwrap();
                get_msgs_query.set_from_id(Some(next_msg_id));
            });
            match crate::req::get_msgs_with_ctx_err(&client, &get_msgs_query) {
                Ok(msgs) => {
                    messages = msgs;
                    crate::ui::stdstreams::print_msgs(messages.iter());
                }
                Err(err) => tracing::error!("{err}"),
            }

            // Calculate elapsed time and sleep for the remaining time to make it 1 second
            let elapsed = chrono::Utc::now().signed_duration_since(start);
            let elapsed_ms = elapsed.num_milliseconds();
            if elapsed_ms < 1000 {
                std::thread::sleep(Duration::from_millis((1000 - elapsed_ms) as u64));
            }
        }
    })
}
