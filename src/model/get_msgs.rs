use serde::Serialize;

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
            from_id.clone(),
            to_id.clone(),
            from_time.clone(),
            to_time.clone(),
        )
    }
}
