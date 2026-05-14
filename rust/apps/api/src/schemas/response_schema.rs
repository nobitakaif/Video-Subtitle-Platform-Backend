use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserSignupReponse {
    pub user_id : String 
}

#[derive(Serialize, Deserialize)]
pub struct UserSigninResponse{
    pub token : String
}

#[derive(Serialize,Deserialize)]
pub struct UserSigninFailed{
    pub message : String,
}

#[derive(Serialize)]
#[serde(tag = "status", content = "data")]
pub enum SigninResponse {
    Ok(UserSigninResponse),
    Err(UserSigninFailed),
}