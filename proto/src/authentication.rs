use prost::Message;
pub mod security {

    include!(concat!(env!("OUT_DIR"), "/security.rs"));
}

// #[derive(Clone, PartialEq, Message)]
// pub struct LoginPayload {
//     #[prost(string, tag="1")]
//     pub client_id: String,
//     #[prost(string, tag="2")]
//     pub  client_secret: String,
// }

// impl LoginPayload {
//     pub fn encode_payload(&self) -> Vec<u8> {
//         let mut buf = Vec::new();
//         buf.reserve(self.encoded_len());
//         self.encode(&mut buf).unwrap();
//         buf
//     }
// }

impl security::LoginPayload {
    pub fn encode_payload(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

// pub fn build_payload() -> Result<security::LoginPayload, core::MarvelError> {
//     let v = security::LoginPayload {
//         userid: "user1".to_string(),
//         pwd: "pass".to_string(),
//     };
//     Ok(v)
// }
//
// pub fn serialize_payload(login_payload: &security::LoginPayload) -> Vec<u8> {
//     let mut buf = Vec::new();
//     buf.reserve(login_payload.encoded_len());
//
//     login_payload.encode(&mut buf).unwrap();
//     buf
// }
