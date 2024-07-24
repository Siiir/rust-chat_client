use serde::Serialize;

pub fn get_msgs(
    client: &reqwest::blocking::Client,
    get_msgs_query: &crate::req::GetMsgs,
) -> anyhow::Result<Vec<crate::model::ChatMsg>> {
    let url = format!("http://{}/v1/msg", crate::app::cfg().server_addr);
    let resp = client
        .get(url)
        .query(get_msgs_query)
        .send();
    crate::req::interpret_resp(resp)
}

/// <How much & where to> move the `from_id` cursor relatively to the recent read?
pub const UNREAD_MSG_OFFSET: i64 = -5;

#[derive(Default, derive_more::Constructor, Debug, Serialize)]
pub struct GetMsgs {
    from_id: Option<i64>,
    to_id: Option<i64>,
    from_time: Option<chrono::DateTime<chrono::Local>>,
    to_time: Option<chrono::DateTime<chrono::Local>>,
}
impl GetMsgs {
    // Getters
    pub fn from_id(&self) -> Option<i64> {
        self.from_id
    }
    pub fn to_id(&self) -> Option<i64> {
        self.to_id
    }
    pub fn from_time(&self) -> Option<&chrono::DateTime<chrono::Local>> {
        self.from_time.as_ref()
    }
    pub fn to_time(&self) -> Option<&chrono::DateTime<chrono::Local>> {
        self.to_time.as_ref()
    }

    // Setters
    pub fn set_from_id(&mut self, from_id: Option<i64>) {
        self.from_id = from_id;
    }
    pub fn set_to_id(&mut self, to_id: Option<i64>) {
        self.to_id = to_id;
    }
    pub fn set_from_time(&mut self, from_time: Option<chrono::DateTime<chrono::Local>>) {
        self.from_time = from_time;
    }
    pub fn set_to_time(&mut self, to_time: Option<chrono::DateTime<chrono::Local>>) {
        self.to_time = to_time;
    }
}

impl From<&crate::cli::Args> for GetMsgs {
    fn from(args: &crate::cli::Args) -> Self {
        let crate::cli::Args {
            from_id,
            to_id,
            from_time,
            to_time,
            ..
        } = args;

        Self::new(
            Some(from_id.unwrap_or_else(|| {
                let int = crate::pa::read::future_msg_id();
                int + UNREAD_MSG_OFFSET
            })),
            to_id.clone(),
            from_time.clone(),
            to_time.clone(),
        )
    }
}
