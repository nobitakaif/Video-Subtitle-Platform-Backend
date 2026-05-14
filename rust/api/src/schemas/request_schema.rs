use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize)]
pub struct UserSigninBody{
    pub name : String,
    pub email : String,
    pub passsword : String
}