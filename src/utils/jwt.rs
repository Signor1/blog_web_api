use chrono::{Duration, Utc};

pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32,
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, ()> {
    let now = Utc::now();
    let expiry = Duration::hours(24);

    let claims = Claims{
        exp: (now+expiry).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id
    }
}
