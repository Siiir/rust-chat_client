pub use args::Args;
pub mod args {
    use std::net::{IpAddr, Ipv4Addr};

    #[derive(clap::Parser, Debug)]
    #[command(version, about)]
    pub struct Args {
        /// Address of the targeted server.
        #[arg(short='a', long, default_value_t = IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)))]
        pub ip_addr: IpAddr,
        /// Port of the targeted server.
        #[arg(short = 'p', long, default_value_t = 8080)]
        pub port: u16,
        /// User nickname for the chatting purposes.
        #[arg(short = 'n', long, default_value = "anon")]
        pub nickname: String,

        #[arg(short = 'i', long)]
        pub from_id: Option<i64>,
        #[arg(short = 'I', long)]
        pub to_id: Option<i64>,
        #[arg(short = 't', long)]
        pub from_time: Option<chrono::DateTime<chrono::Local>>,
        #[arg(short = 'T', long)]
        pub to_time: Option<chrono::DateTime<chrono::Local>>,
    }
}
