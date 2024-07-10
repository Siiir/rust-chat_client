use chat_client::model::{ChatMsg, GetMsgs};
use reqwest::blocking::Client;

fn main() -> anyhow::Result<()> {
    // Inits – basic
    chat_client::init::all();
    let cli_args: chat_client::cli::Args = clap::Parser::parse();
    let mut get_msgs_query: GetMsgs = (&cli_args).into();
    // Inits – app state
    if get_msgs_query.from_id().is_none() {
        let int = chat_client::pa::read::future_msg_id();
        get_msgs_query.set_from_id(Some(int - 5));
    };
    let client = Client::new();

    // Fetching requested messages as the first automatic user action.
    let messages: Vec<ChatMsg> = chat_client::req::get_msgs_with_ctx_err(&client, &get_msgs_query)?;
    chat_client::ui::stdstreams::print_msgs(messages.iter());

    // Starting message featching deamon
    if cli_args.to_id.is_none() {
        chat_client::start_msg_fetching_deamon(messages, client.clone());
    }
    // Posting loop
    let mut msg_content = String::new();
    loop {
        std::io::stdin().read_line(&mut msg_content).unwrap();
        match chat_client::req::post_msg_with_ctx_err(
            &client,
            &chat_client::model::PostMsg::new(
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
