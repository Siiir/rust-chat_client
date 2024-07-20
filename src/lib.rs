pub mod model;
use model::ChatMsg;
pub use ui::cli;
pub mod init;
pub mod pa;
pub mod req;
pub use req::ReqClient;
pub mod ui;
pub mod util;

pub type MsgId = i64;

pub fn pull_msgs(client: &ReqClient, get_msgs_query: crate::req::GetMsgs) -> Result<Vec<ChatMsg>, anyhow::Error> {
    let messages: Vec<ChatMsg> = crate::req::get_msgs_with_ctx_err(client, &get_msgs_query)?;
    crate::ui::stdstreams::print_msgs(messages.iter());
    Ok(messages)
}


pub fn msg_posting_loop(client: &ReqClient, cli_args: &crate::cli::Args) -> ! {
    let mut msg_content = String::new();
    loop {
        std::io::stdin().read_line(&mut msg_content).unwrap();
        match crate::req::post_msg_with_ctx_err(
            &client,
            &crate::req::PostMsg::new(
                cli_args.nickname.clone(),
                msg_content.trim_end().to_owned(),
            ),
        ) {
            /*Ignore as it will appear on concole anyway, thanks to the featching deamon.*/
            Ok(_posted_msg) => (),
            Err(e) => {
                println!("Encountered error when sending your message to the server. It might not have been delivered. Check logs for more details.");
                tracing::error!("{e}");
            }
        };
        msg_content.clear();
    }
}

pub fn start_msg_fetching_deamon(
    mut messages: Vec<model::ChatMsg>,
    client: reqwest::blocking::Client,
) -> std::thread::JoinHandle<impl Send + 'static> {
    use std::time::Duration;

    std::thread::spawn(move || {
        let mut get_msgs_query = req::GetMsgs::default();
        loop {
            let start = chrono::Utc::now();

            // Perform query
            messages.last().map(|msg| msg.id() + 1).map(|future_msg_id| {
                crate::pa::write::future_msg_id_throwing_ctx_err(future_msg_id).unwrap();
                get_msgs_query.set_from_id(Some(future_msg_id));
            });
            match crate::req::get_msgs_with_ctx_err(&client, &get_msgs_query) {
                Ok(msgs) => {
                    messages = msgs;
                    crate::ui::stdstreams::print_msgs(messages.iter());
                }
                Err(err) => tracing::error!("{err:?}\n"),
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
