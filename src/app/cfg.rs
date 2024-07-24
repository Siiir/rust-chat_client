use std::{net::SocketAddr, sync::OnceLock};

use crate::cli::Args;

pub fn get() -> &'static crate::AppCfg {
    APP_CFG.get()
        .unwrap_or_else(|| panic!("Logical error: App config used before initialization."))
}
pub fn set(val: AppCfg) {
    APP_CFG.set(val)
        .unwrap_or_else(|_old_val| panic!("Logical error: Attempted 2nd initialization of app config."))
}

static APP_CFG: OnceLock<AppCfg> = OnceLock::new();

pub struct AppCfg {
    pub server_addr: SocketAddr,
}

// CRUD-C: Constructors
impl From<&Args> for AppCfg {
    fn from(&Args { ip_addr, port, .. }: &Args) -> Self {
        Self {
            server_addr: SocketAddr::new(ip_addr, port),
        }
    }
}
