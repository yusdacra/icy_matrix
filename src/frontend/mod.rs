pub mod member;
pub mod message;
pub mod node;

pub use action::*;
pub use member::*;
pub use message::*;
pub use node::*;

#[derive(Debug, Clone)]
pub enum AuthStatus {
    None,
    Member { member: Member },
}

impl AuthStatus {
    pub fn is_authenticated(&self) -> bool {
        match self {
            None => false,
            _ => true,
        }
    }
}

pub mod action {
    use super::*;
    use std::{collections::HashMap, time::Duration};

    #[derive(Debug, Clone)]
    pub enum Action {
        /// Logout, must delete any session "cookie".
        Logout,
        /// Login or register.
        Auth(HashMap<String, String>),
        /// Send message(s) in the specified room(s).
        SendMessage {
            node_id: Vec<String>,
            message: Vec<Message>,
        },
    }

    impl Into<PendingAction> for Action {
        fn into(self) -> PendingAction {
            PendingAction {
                action: self,
                remaining_repeat_count: 0,
                repeat_after: None,
                error: None,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct PendingAction {
        pub action: Action,
        pub remaining_repeat_count: u8,
        pub repeat_after: Option<Duration>,
        pub error: Option<String>,
    }
}
