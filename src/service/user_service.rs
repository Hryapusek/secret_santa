use super::database_connection::Database;
use crate::models::user::*;

use diesel::prelude::*;

pub struct UserService {
    pub conn: diesel::PgConnection,
}

impl UserService {
    pub fn new() -> UserService {
        UserService {
            conn: (Database::connect()),
        }
    }

    pub fn create_user(&mut self, username: &String) -> Result<User, ()> {
        match self.get_user_by_name(username) {
            None => {}
            Some(..) => {
                println!("User with this name already exists!");
                return Err(());
            }
        }
        let new_user = NewUser::new(String::clone(username));
        use crate::schema::users::dsl::*;
        let user = diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&mut self.conn);
        match user {
            Ok(u) => {
                println!("Created user with id: {}, name: {}", u.id, u.name);
                Ok(u)
            }
            Err(..) => {
                println!("Error occured putting user {} in database", username);
                Err(())
            }
        }
    }
    fn get_user_by_name(&mut self, username: &String) -> Option<User> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(ExpressionMethods::eq(name, username))
            .first::<User>(&mut self.conn);
        match user {
            Ok(u) => Some(u),
            Err(..) => None,
        }
    }

}