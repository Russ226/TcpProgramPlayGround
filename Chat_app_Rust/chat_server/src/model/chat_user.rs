#[derive(Debug, Clone)]
pub struct ChatUser {
    pub ip_addr: String,
    pub username: String
}

impl ChatUser{
    pub fn new(addr: String, username: String) -> Self {
        return ChatUser {ip_addr: addr, username: username};
    }
}

impl PartialEq for ChatUser {
    fn eq(&self, right: &Self) -> bool {
        return self.ip_addr == right.ip_addr && self.username == right.username;
    }
}

