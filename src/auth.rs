use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::auth::AuthError::{UserDoesNotExist, WrongPassword};
use crate::{database, models};

static RANDOM_HASH: &str = "$argon2i$v=19$m=16,t=2,p=1$OGxkbkpGcHZWTzlkNU00WQ$FTyekoCrFSYE08v5FLA8N";

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use crate::models::User;

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
        return verify_password(password, (*user).get_password().as_str())
            .map(|_| ()).or(Err(WrongPassword));
    }
    verify_password(password, RANDOM_HASH).ok();
    Err(UserDoesNotExist)
}

fn hash_password(password: &str) -> Result<String, ()> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let hashed_password = argon2.hash_password(password.as_bytes(), &salt).or(Err(()))?.to_string();

    Ok(hashed_password)
}

fn verify_password(password: &str, hashed_password: &str) -> Result<(), ()> {
    let parsed_hash = PasswordHash::new(&hashed_password).or(Err(()))?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).or(Err(()))
}

pub fn hash_password_inplace<T: User>(mut user: T) -> T {
    let hashed_password = hash_password(user.get_password().as_str()).unwrap();
    user.set_password(hashed_password.as_str());
    user
}

#[test]
fn hash_password_test() {
    let result = hash_password("1234");
    if let Ok(hashed_password) = result {
        println!("{}", hashed_password);
        if let Ok(_) = verify_password("1234", hashed_password.as_str()) {
            return;
        }
    }

    panic!();
}