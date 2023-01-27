use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::auth::AuthError::{UserDoesNotExist, WrongPassword};
use crate::{database, models};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[derive(Debug)]
pub enum AuthError {
    WrongPassword,
    UserDoesNotExist,
}

impl Error for AuthError {}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WrongPassword => {
                write!(f, "Wrong password")
            }
            UserDoesNotExist => {
                write!(f, "User does not exist")
            }
        }
    }
}


pub fn login_as_student(db: &database::DB, username: &str, password: &str) -> Result<(), AuthError> {
    login(db.students.lock().unwrap().borrow_mut(), username, password)
}

pub fn login_as_teacher(db: &database::DB, username: &str, password: &str) -> Result<(), AuthError> {
    login(db.teachers.lock().unwrap().borrow_mut(), username, password)
}

fn login(users: &mut HashMap<String, impl models::User>, username: &str, password: &str) -> Result<(), AuthError> {
    if let Some(user) = users.get_mut(username) {
        if password == (*user).get_password() {
            return Ok(());
        }
        return Err(WrongPassword);
    }
    Err(UserDoesNotExist)
}

fn hash_password(password: &str) -> Result<String, ()>{
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let hashed_password = argon2.hash_password(password.as_bytes(), &salt).or(Err(()))?.to_string();

    Ok(hashed_password)
}

fn verify_password(password: &str, hashed_password:&str) -> Result<(), ()> {
    let parsed_hash = PasswordHash::new(&hashed_password).or(Err(()))?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).or(Err(()))
}

#[test]
fn hash_password_test(){
    let result = hash_password("1234");
    if let Ok(hashed_password) = result {
        println!("{}", hashed_password);
        if let Ok(_) = verify_password("1234", hashed_password.as_str()) {
            return;
        }
    }

    panic!();
}