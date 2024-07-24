pub mod cli;
pub mod stdstreams {
    use crate::model::ChatMsg;

    pub fn print_msgs<'m, II: IntoIterator<Item = &'m ChatMsg>>(messages: II) {
        for message in messages {
            println!("{:}", message);
        }
    }
}
