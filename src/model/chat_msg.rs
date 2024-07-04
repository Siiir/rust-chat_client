use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, derive_more::Display)]
#[display(
    fmt = "{id} | {} | {author}: {content}",
    r#"date_time.format("%y-%m-%d %a %H:%M")"#
)]
pub struct ChatMsg {
    id: i64,
    author: String,
    date_time: chrono::DateTime<chrono::Local>,
    content: String,
}

impl ChatMsg {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn author(&self) -> &str {
        &self.author
    }
    pub fn date_time(&self) -> &chrono::DateTime<chrono::Local> {
        &self.date_time
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::model::ChatMsg;

    #[test]
    fn deser_with_offset_then_display() {
        use serde_json::from_value;

        let msg_with_offset_of_4: ChatMsg = from_value(json!({
            "author": "Tomek",
            "content": "Lubie placki",
            "date_time": "2024-06-25T19:22:07.707544+04:00",
            "id": 7
        }))
        .expect("JSON literal should be deserializable as a chat message.");

        let displayed = format!("{}", msg_with_offset_of_4);
        assert_eq!(&displayed, "7: at 24-06-25 Tue 17:22 Tomek: Lubie placki");
    }
}
