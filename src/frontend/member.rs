use http::Uri;

#[derive(Debug, Default, Clone)]
pub struct Member {
    pub id: String,
    pub avatar_url: Option<Uri>,
    pub name: String,
    pub presence: Presence,
    pub room_presence: RoomPresence,
}

impl Member {
    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }

    pub fn with_avatar(mut self, url: Uri) -> Self {
        self.avatar_url = Some(url);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_presence(mut self, presence: Presence) -> Self {
        self.presence = presence;
        self
    }

    pub fn with_room_presence(mut self, room_presence: RoomPresence) -> Self {
        self.room_presence = room_presence;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Presence {
    Typing,
    Online,
    Idle,
    DND, // Do not disturb
    Offline,
}

impl Default for Presence {
    fn default() -> Self {
        Presence::Offline
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RoomPresence {
    Joined,
    Invited { by_member: Box<Member> },
    Left,
}

impl Default for RoomPresence {
    fn default() -> Self {
        RoomPresence::Joined
    }
}
