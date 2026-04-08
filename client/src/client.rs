use proto::authentication::security;
use core::MarvelError;


pub async fn authenticate(username: &str, password: &str) -> Result<(), MarvelError> {
    // Dummy authentication logic
    let payload = security::LoginPayload {
        userid: username.to_string(),
        pwd: password.to_string(),
    };

    let data: Vec<u8> = payload.encode_payload();

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3000/authenticate")
        .header("Content-Type", "application/protobuf")
        .body(data)
        .send()
        .await;
    // .map_err(|e| MarvelError::ReqwestError(e))
    return match res {
        Ok(response) => {
            dbg!(response);
            Ok(())
        }
        Err(e) => Err(MarvelError::NetworkError(e.to_string())),
    };
}
