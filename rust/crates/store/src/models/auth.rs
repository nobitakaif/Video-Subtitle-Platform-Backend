use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable,Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct UserSignup{
    id : Uuid, 
    name : String, 
    email : String,
    password : String
}

struct UserSignin{
    name : String, 
    password : String, 
}

impl Store{
    pub fn user_signup(&mut self, name : String, email : String, password : String ) -> Result<Uuid , diesel::result::Error>{
        let user = UserSignup{
            id : uuid::Uuid::new_v4(),
            name,
            email, 
            password,
        };

        let res = diesel::insert_into(crate::schema::users::table)
                .values(&user)
                .returning(UserSignup::as_returning())
                .get_result(&mut self.pool)?;
        
        Ok(res.id)

    }
    pub fn user_signin(&self){

    }
}