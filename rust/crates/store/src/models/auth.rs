use crate::store::Store;
use diesel;
use schema::user;

#[diesel(table_name = user)]

struct UserSignup{
    id : String, 
    name : String, 
    password : String 
}

struct UserSignin{
    name : String, 
    password : String 
}

impl Store{
    pub fn user_signup(&self){
        
    }
    pub fn user_signin(&self){

    }
}