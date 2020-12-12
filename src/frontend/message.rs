use super::member::*;
use http::Uri;

#[derive(Debug, Default, Clone)]
pub struct Message {
    id: String,
    body: String,
    sender: Member,
    attachment: Option<Uri>,
    reply_to: Option<Box<Message>>,
}

impl Message {
    pub fn new(id: String, sender: Member, body: String) -> Self {
        Self {
            id,
            body,
            sender,
            ..Self::default()
        }
    }

    pub fn add_attachment(mut self, attachment: Uri) -> Self {
        self.attachment = Some(attachment);
        self
    }

    pub fn in_reply_to(mut self, msg: Message) -> Self {
        self.reply_to = Some(msg);
        self
    }
}
