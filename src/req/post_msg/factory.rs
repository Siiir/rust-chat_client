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
