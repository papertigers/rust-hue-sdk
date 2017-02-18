/// Hue User
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
}

impl User {
    /// Returns a hue User 
    pub fn new(uname: &str) -> User {
        User {
            username: uname.to_owned(),
        }
    }
}
