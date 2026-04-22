use std::fmt::Display;
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

impl security::AuthenticateRequest {
    pub fn encode_payload(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl security::AuthenticateResponse {
    pub fn decode_payload(bytes: impl AsRef<[u8]>) -> Result<Self, prost::DecodeError> {
        Self::decode(bytes.as_ref())
    }
}

impl security::AuthorizeRequest {
    pub fn encode_payload(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl security::AuthorizeResponse {
    pub fn decode_payload(bytes: impl AsRef<[u8]>) -> Result<Self, prost::DecodeError> {
        Self::decode(bytes.as_ref())
    }
}

impl Display for security::AuthorizeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthorizeRequest {{ grant_type: {}, client_id: {}, scope: {}, refresh_token: {}, code_verifier: {}, code: {}, redirect_uri: {} }}",
            self.grant_type, self.client_id, self.scope, self.refresh_token, self.code_verifier, self.code, self.redirect_uri)
    }
}