use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::{env, str};
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
        secret: &str,
        nonce: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let json = decrypt_bytes(
            &buffer,
            <&[u8; 32]>::try_from(secret.as_bytes()).unwrap(),
            <&[u8; 24]>::try_from(nonce.as_bytes()).unwrap(),
        );

        let db: DB = serde_json::from_str(json.as_str())?;

        Ok(db)
    }

    pub fn from_env() -> Self {
        let (teachers, students) = read_users_from_env();

        let teachers_map: HashMap<String, Teacher> =
            teachers.into_iter().map(|v| (v.creds.username.clone(), v))
                .collect();
        let students_map: HashMap<String, Student> =
            students.into_iter().map(|v| (v.creds.username.clone(), v))
                .collect();

        Self {
            teachers: Mutex::new(teachers_map),
            students: Mutex::new(students_map),
        }
    }

    pub fn save(&self, file_path: &str, secret: &str, nonce: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path).unwrap();
        let mut writer = BufWriter::new(file);
        //serde_json::to_writer(writer, self)?;
        let json = serde_json::to_string(self)?;
        let data = encrypt_str(json.as_str(),
                               <&[u8; 32]>::try_from(secret.as_bytes()).unwrap(),
                               <&[u8; 24]>::try_from(nonce.as_bytes()).unwrap(),
        );

        writer.write(&*data)?;
        Ok(())
    }
}

fn encrypt_str(text: &str, key: &[u8; 32], nonce: &[u8; 24]) -> Vec<u8> {

    // Generate a random secret key and nonce
    let secret_key = Key::from(key);
    let nonce = Nonce::from(nonce);

    // Encrypt `message`, into a Vec-based box
    let dryocsecretbox = DryocSecretBox::encrypt_to_vecbox(text.as_bytes(), &nonce, &secret_key);

    dryocsecretbox.to_vec()
}

fn decrypt_bytes(bytes: &Vec<u8>, key: &[u8; 32], nonce: &[u8; 24]) -> String {
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

fn read_users_from_env() -> (Vec<Teacher>, Vec<Student>) {
    let mut teachers = Vec::new();
    let mut students = Vec::new();

    let mut index = 0;
    loop {
        index += 1;
        if let Ok(username) = env::var(format!("TEACHER_USERNAME_{}", index)) {
            teachers.push(Teacher { creds: Credentials { username, password: env::var(format!("TEACHER_PASSWORD_{}", index)).unwrap() } });
        } else {
            index = 0;
            break;
        }
    };

    loop {
        index += 1;
        if let Ok(username) = env::var(format!("STUDENT_USERNAME_{}", index)) {
            students.push(Student { creds: Credentials { username, password: env::var(format!("STUDENT_PASSWORD_{}", index)).unwrap() }, grades: vec![] });
        } else {
            break;
        }
    };


    (teachers.into_iter().map(super::auth::hash_password_inplace).collect(),
     students.into_iter().map(super::auth::hash_password_inplace).collect())
}

#[test]
fn encrypt_test() {
    let secret_key = "7ed049e344f73f399ba1f7868cf9494f";
    let nonce = "7ed049e344f73f399ba1f786";
    encrypt_str("lol", <&[u8; 32]>::try_from(secret_key.as_bytes()).unwrap(), <&[u8; 24]>::try_from(nonce.as_bytes()).unwrap());
}