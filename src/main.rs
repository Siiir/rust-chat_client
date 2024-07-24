use reqwest::blocking::Client;

fn main() -> anyhow::Result<()> {
    // Inits – RO objects
    let (cli_args, get_msgs_query) = chat_client::init::all();
    // Inits – app state
    let client = Client::new();

    // Fetching requested messages as the first automatic client action.
    let future_msg_id = chat_client::app::pull_msgs(&client, get_msgs_query)?
        .last()
        .map(chat_client::model::ChatMsg::id)
        .map(|last_msg_id| last_msg_id + 1);

    // Starting message featching deamon
    if cli_args.to_id.is_none() {
        chat_client::app::start_msg_fetching_thread(client.clone(), future_msg_id);
    }
    // Posting loop
    chat_client::app::msg_posting::run_loop(
        &client,
        &chat_client::MsgPoster::new(cli_args.nickname),
    )
}

