pub fn run_loop(client: &crate::ReqClient, msg_poster: &crate::MsgPoster) -> ! {
    let mut msg_content = String::new();
    loop {
        std::io::stdin().read_line(&mut msg_content).unwrap();
        match crate::req::post_msg_with_ctx_err(
            &client,
            &msg_poster.craft(msg_content.trim_end().to_owned()),
        ) {
            /*Ignore as the message will appear on concole anyway, thanks to the featching deamon.*/
            Ok(_posted_msg) => (),
            Err(e) => {
                handle_posting_err(e);
            }
        };
        msg_content.clear();
    }
}

fn handle_posting_err(e: anyhow::Error) {
    println!("Encountered error when sending your message to the server. It might not have been delivered. Check logs for more details.");
    tracing::error!("{e}");
}
