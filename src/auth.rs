use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::auth::AuthError::{UserDoesNotExist, WrongPassword};
use crate::{database, models};

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