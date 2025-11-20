mod security {

    include!(concat!(env!("OUT_DIR"), "/security.rs"));
}

use security;

use crate::security;
pub fn test() -> Result<(), std::error::Error> {
    let v = security::LoginRequest {
        userid: "user1".to_string(),
        pwd: "pass".to_string(),
    };
    Ok(())
}
