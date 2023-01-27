use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Teacher {
    pub creds: Credentials,
}

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub creds: Credentials,
    pub grades: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub trait User {
    fn get_username(&self) -> String;
    fn get_password(&self) -> String;
    fn set_username(&mut self, username: &str);
    fn set_password(&mut self, password: &str);
}

macro_rules! impl_user_trait {
    ($a:ty)=>{
        impl User for $a {
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
    }
}

impl_user_trait!(Student);
impl_user_trait!(Teacher);