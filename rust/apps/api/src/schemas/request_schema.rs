use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct UserSignupBody{
    pub name : String, 
    pub email : String,
    pub password : String
}

#[derive(Serialize, Deserialize)]
pub struct UserSigninBody{
    pub email : String, 
    pub password : String
}