use bcrypt::{hash, DEFAULT_COST};
use rusqlite::{self, Connection, Error, OptionalExtension};
use serde::Deserialize;
use bcrypt::verify;

#[derive(Deserialize, Debug)]
pub struct UserSignIn {
    pub email_sign_in: String,
    pub password_sign_in: String,
}

#[derive(Deserialize, Debug)]
pub struct UserCheck{
    pub email_check: String,
}

#[derive(Deserialize, Debug)]
pub struct UserSignUp {
    pub email_sign_up: String,
    pub password_sign_up: String,
    pub password_confirmation: String,
}


pub trait DataBaseFunctions{
    fn add_user(&self, user: UserSignUp)-> Result<(), Error>;
    fn check_user_sign_in(&self, user: UserSignIn)->Result<bool, Error>;
    fn check_user_forgot_password(&self, user: UserCheck)-> Result<bool, Error>;
}

impl DataBaseFunctions for Connection{

    fn add_user(&self, user: UserSignUp) -> Result<(), Error> {
        let hashed_password = hash(user.password_sign_up.as_bytes(), DEFAULT_COST)
        .map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;

        self.execute(
            "INSERT INTO user (email, password) VALUES (?1, ?2)", 
            rusqlite::params![user.email_sign_up, hashed_password]
        )?;
        Ok(())
    }

    fn check_user_sign_in(&self, user: UserSignIn)->Result<bool, Error> {
        
        let hashed_password: Option<String> = self.query_row(
        "SELECT password FROM user WHERE email = ?1",
        rusqlite::params![user.email_sign_in],
        |row| row.get(0)
        ).optional()?;

        match hashed_password {
            Some(hash) => verify(user.password_sign_in.as_bytes(), &hash)
            .map_err(|e| Error::ToSqlConversionFailure(Box::new(e))),
            None => Ok(false),
            }
        }

    fn check_user_forgot_password(&self, user: UserCheck) -> Result<bool, Error> {
        
        // Changed "users" to "user" to match your table name
        let exists: Option<bool>= self.query_row(
            "SELECT * FROM user WHERE email = ?1",
            rusqlite::params![user.email_check],
            |row| row.get(0),
        ).optional()?;

        println!("Here {:?}", &exists);

        match exists {
            Some(bool) => Ok(bool),
            None => Ok(false),
        }
    }

}

pub fn redirect_to_page(){
    
}

pub fn connect_to_db()-> Result<Connection, Error>{

    let db = rusqlite::Connection::open("db.db")?;

    create_tables(&db);
    
    Ok(db)
}

fn create_tables(db: &Connection){

    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS user
        (id INTEGER PRIMARY KEY AUTOINCREMENT, 
        email VARCHAR(255) UNIQUE, 
        password TEXT);
            ",
    ).expect("Failed to create tables");

}
