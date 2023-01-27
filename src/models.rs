use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Teacher {
    pub(crate) creds: Credentials,
}

#[derive(Serialize, Deserialize)]
pub struct Student {
    creds: Credentials,
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
}

impl User for Teacher {
    fn get_username(&self) -> String {
        self.creds.username.clone()
    }

    fn get_password(&self) -> String {
        self.creds.password.clone()
    }
}

impl User for Student {
    fn get_username(&self) -> String {
        self.creds.username.clone()
    }

    fn get_password(&self) -> String {
        self.creds.password.clone()
    }
}