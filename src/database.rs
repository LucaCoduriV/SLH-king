use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::{env, fs, str};
use crate::models::{Credentials, Student, Teacher};
use dryoc::dryocsecretbox::*;

#[derive(Serialize, Deserialize, Default)]
pub struct DB {
    pub teachers: Mutex<HashMap<String, crate::models::Teacher>>,
    pub students: Mutex<HashMap<String, crate::models::Student>>,
}

impl DB {
    #[allow(dead_code)]
    pub fn new(students: Mutex<HashMap<String, crate::models::Student>>, teachers: Mutex<HashMap<String, crate::models::Teacher>>) -> Self {
        DB {
            teachers,
            students,
        }
    }

    pub fn from_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let db: DB = serde_json::from_reader(reader).unwrap_or(Self::default());

        if let Ok(ref mut teachers) = db.teachers.lock() {
            teachers.insert("Danono".to_string(), crate::models::Teacher { creds: Credentials { username: "Danono".to_string(), password: "1234".to_string() } });
            teachers.insert("Duc".to_string(), crate::models::Teacher { creds: Credentials { username: "Duc".to_string(), password: "l4crypt0C3stR1g0l0".to_string() } });
        }

        if let Ok(ref mut students) = db.students.lock() {
            students.insert("Luca".to_string(), crate::models::Student { creds: Credentials { username: "Luca".to_string(), password: "1234".to_string() }, grades: Vec::new() });
            students.insert("Chloé".to_string(), crate::models::Student { creds: Credentials { username: "Chloé".to_string(), password: "1234".to_string() }, grades: Vec::new() });
        }

        Ok(db)
    }

    pub fn save(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;

        Ok(())
    }
}

impl Default for DB {
    fn default() -> Self {
        let (mut teachers, mut students) = read_users_from_env();

    }
}

fn encrypt_str(text: &str, key:&[u8;32], nonce:&[u8;24]) -> Vec<u8> {

    // Generate a random secret key and nonce
    let secret_key = Key::from(key);
    let nonce = Nonce::from(nonce);

    // Encrypt `message`, into a Vec-based box
    let dryocsecretbox = DryocSecretBox::encrypt_to_vecbox(text.as_bytes(), &nonce, &secret_key);

    dryocsecretbox.to_vec()
}

fn decrypt_bytes(bytes: &Vec<u8>, key:&[u8;32], nonce:&[u8;24]) -> String{
    // Read the same box we just made into a new DryocBox
    let dryocsecretbox = DryocSecretBox::from_bytes(bytes).expect("unable to load box");

    let secret_key = Key::from(key);
    let nonce = Nonce::from(nonce);

    // Decrypt the box we previously encrypted,
    let decrypted = dryocsecretbox
        .decrypt_to_vec(&nonce, &secret_key)
        .expect("unable to decrypt");

    String::from_utf8(decrypted.to_vec()).unwrap()
}

fn read_users_from_env() -> (Vec<Teacher>, Vec<Student>){
    let mut teachers = Vec::new();
    let mut students = Vec::new();

    let mut index = 0;
    loop {
        index += 1;
        if let Ok(username) = env::var(format!("TEACHER_USERNAME_{}", index)){
            teachers.push(Teacher{creds:Credentials{username, password: env::var(format!("TEACHER_PASSWORD_{}", index)).unwrap()}});
        }else{
            index = 0;
            break;
        }
    };

    loop {
        index += 1;
        if let Ok(username) = env::var(format!("STUDENT_USERNAME_{}", index)){
            students.push(Student{creds:Credentials{username, password: env::var(format!("STUDENT_PASSWORD_{}", index)).unwrap()}, grades: vec![] });
        }else{
            break;
        }
    };

    (teachers, students)
}

#[test]
fn encrypt_test(){
    let secret_key = "7ed049e344f73f399ba1f7868cf9494f";
    let nonce = "7ed049e344f73f399ba1f786";
    encrypt_str("lol", <&[u8; 32]>::try_from(secret_key.as_bytes()).unwrap(), <&[u8; 24]>::try_from(nonce.as_bytes()).unwrap());
}