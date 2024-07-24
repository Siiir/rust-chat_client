pub mod model;

pub mod init;

pub mod pa;

pub use req::{post_msg::factory::MsgPoster, ReqClient};
pub mod req;

pub use ui::cli;
pub mod ui;

pub mod util;

pub use app::cfg::AppCfg;
pub mod app;

pub type MsgId = i64;
