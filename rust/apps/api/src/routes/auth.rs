use axum::{Json};
use store::Store;
use crate::schemas::response_schema::{SigninResponse, UserSigninFailed, UserSigninResponse};
use crate::schemas::{request_schema,response_schema};
use request_schema::{UserSigninBody, UserSignupBody};
use response_schema::{UserSignupReponse};


pub async fn signup_handler(Json(body) : Json<UserSignupBody>) ->String{
   let UserSignupBody {name, email, password} = body;

   // put this data into db and password should be hashed 
   Store::create_user(&Store{
      connection : String::from("alight")
   });
   format!("signin done!")
   
} 

pub async fn signin_handler(Json(body) : Json<UserSigninBody>) -> Json<SigninResponse>{

   let UserSigninBody {email, password} = body;

   // check this user exist or not into db if not then send msg user not found or incorrect email and return 
   // if use does exist then compare the plain password with hashedpassword if miss matched then send msg incorrect password and return 
   // if password is also correct then generate a jwt token for this user_id and return to them
   let success = true;
   if success {
      Json(SigninResponse::Ok(UserSigninResponse {
         token : String::from("token11111222222223333333333")
      }))
   }else{
      Json(SigninResponse::Err(UserSigninFailed{
         message : String::from("Invalied credential")
      }))
   }
}