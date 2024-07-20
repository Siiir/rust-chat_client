use serde::Serialize;

pub mod factory{
    use super::PostMsg;
    use derive_more as dm;

    #[derive(dm::Constructor)]
    pub struct MsgPoster{
        name: String,
    }
    impl MsgPoster{
        pub fn craft(&self, content: String) -> PostMsg{
            PostMsg { author: self.name.clone(), content }
        }
    }
}

#[derive(derive_more::Constructor, Debug, Serialize)]
pub struct PostMsg {
    author: String,
    content: String,
}
