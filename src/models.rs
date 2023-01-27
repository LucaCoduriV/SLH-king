use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Teacher {
    pub(crate) creds: Credentials,
}

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub(crate) creds: Credentials,
    pub grades: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Credentials {
    pub(crate) username: String,
    pub(crate) password: String,
}

pub trait User {
    fn get_username(&self) -> String;
    fn get_password(&self) -> String;
    fn set_username(&mut self, username:&str);
    fn set_password(&mut self, password:&str);
}

impl User for Teacher {
    fn get_username(&self) -> String {
        self.creds.username.clone()
    }

    fn get_password(&self) -> String {
        self.creds.password.clone()
    }

    fn set_username(&mut self, username: &str) {
        self.creds.username = String::from(username);
    }

    fn set_password(&mut self, password: &str) {
        self.creds.password = String::from(password);
    }
}

impl User for Student {
    fn get_username(&self) -> String {
        self.creds.username.clone()
    }

    fn get_password(&self) -> String {
        self.creds.password.clone()
    }

    fn set_username(&mut self, username: &str) {
        self.creds.username = String::from(username);
    }

    fn set_password(&mut self, password: &str) {
        self.creds.password = String::from(password);
    }
}