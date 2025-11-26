mod security {

    include!(concat!(env!("OUT_DIR"), "\\authentication"));
}



pub fn test() -> Result<(), marvel_error::error::MarvelError> {
    let v = security::LoginRequest {
        userid: "user1".to_string(),
        pwd: "pass".to_string(),
    };
    Ok(())
}
