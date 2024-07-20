use chat_client::req::GetMsgs;
use reqwest::blocking::Client;

fn main() -> anyhow::Result<()> {
    // Inits – RO objects
    chat_client::init::all();
    let cli_args: chat_client::cli::Args = clap::Parser::parse();
    let get_msgs_query: GetMsgs = (&cli_args).into();
    // Inits – app state
    let client = Client::new();

    // Fetching requested messages as the first automatic client action.
    let messages = chat_client::app::pull_msgs(&client, get_msgs_query)?;
    // Starting message featching deamon
    if cli_args.to_id.is_none() {
        chat_client::app::start_msg_fetching_thread(messages, client.clone());
    }
    // Posting loop
    chat_client::app::msg_posting::run_loop(&client, &chat_client::MsgPoster::new(cli_args.nickname))
}

