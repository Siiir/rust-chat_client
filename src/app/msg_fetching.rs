use std::time::{self, Duration};

pub const PINGING_INTERVAL: Duration = Duration::from_secs(1);

pub fn pull_msgs(
    client: &crate::ReqClient,
    get_msgs_query: crate::req::GetMsgs,
) -> Result<Vec<crate::model::ChatMsg>, anyhow::Error> {
    let messages: Vec<crate::model::ChatMsg> =
        crate::req::ctxfull::get_msgs(client, &get_msgs_query)?;
    crate::ui::stdstreams::print_msgs(messages.iter());
    Ok(messages)
}

pub fn start_msg_fetching_thread(
    client: reqwest::blocking::Client,
    mut future_msg_id: Option<i64>,
) -> std::thread::JoinHandle<impl Send + 'static> {
    std::thread::spawn(move || {
        let mut get_msgs_query = crate::req::GetMsgs::default();
        loop {
            let start = time::Instant::now();

            // Update `future_msg_id` dependants
            future_msg_id.map(|future_msg_id| {
                crate::pa::write::future_msg_id_throwing_ctx_err(future_msg_id).unwrap();
                get_msgs_query.set_from_id(Some(future_msg_id));
            });
            // Perform the query
            match crate::req::ctxfull::get_msgs(&client, &get_msgs_query) {
                Ok(msgs) => {
                    crate::ui::stdstreams::print_msgs(msgs.iter());
                    // Update [`future_msg_id`]
                    msgs.last()
                        .map(|last_msg| future_msg_id = Some(last_msg.id() + 1));
                }
                Err(err) => tracing::error!("{err:?}\n"),
            }

            sleep_untill_ping_interval_ends(start);
        }
    })
}

pub fn sleep_untill_ping_interval_ends(ping_start: time::Instant) {
    let working_time: Duration = ping_start.elapsed();
    // Nap as a prize for hard work in short time (shorter than [`PINGING_INTERVAL`])
    if let Some(deserved_nap) = PINGING_INTERVAL.checked_sub(working_time) {
        std::thread::sleep(deserved_nap);
    }
}
