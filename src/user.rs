/// Hue User
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
}

impl User {
    /// Returns a hue bridge with the given ip
    pub fn new(username: String) -> User {
        User {
            username: username,
        }
    }
}
