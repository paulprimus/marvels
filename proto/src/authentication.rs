use prost::Message;
pub mod security {

    include!(concat!(env!("OUT_DIR"), "/security.rs"));
}

pub fn build_payload() -> Result<security::LoginPayload, marvel_error::error::MarvelError> {
    let v = security::LoginPayload {
        userid: "user1".to_string(),
        pwd: "pass".to_string(),
    };
    Ok(v)
}

pub fn serialize_payload(login_payload: &security::LoginPayload) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(login_payload.encoded_len());

    login_payload.encode(&mut buf).unwrap();
    buf
}
