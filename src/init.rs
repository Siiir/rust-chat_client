use anyhow::Context;
use tracing_subscriber::FmtSubscriber;

pub fn all() -> (crate::cli::Args, crate::req::GetMsgs) {
    tracing();
    let cli_args: crate::cli::Args = clap::Parser::parse();
    based_on_cli_args(cli_args)
}

fn based_on_cli_args(
    cli_args: crate::cli::Args,
) -> (crate::cli::Args, crate::req::GetMsgs) {
    let get_msgs_query = (&cli_args).into();
    crate::app::cfg::set((&cli_args).into());
    (cli_args, get_msgs_query)
}

fn tracing() {
    // Create a filter that sets the log level for actix to info and the rest to trace
    let env_filter = tracing_subscriber::EnvFilter::new(
        "hyper_util=warn,trace",
    );

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .finish();

    match tracing::subscriber::set_global_default(subscriber).context(
        "Correct logs may not be produced, because app could not set global log subscriber.",
    ) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("WARNING: {e}");
        }
    }
}
