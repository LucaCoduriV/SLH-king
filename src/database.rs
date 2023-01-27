use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::error::Error;
use crate::models::Credentials;


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
            teachers.insert("Danono".to_string(), crate::models::Teacher { creds: Credentials { username: "Danono".to_string(), password: "3lves4ndH0b1ts".to_string() } });
            teachers.insert("Duc".to_string(), crate::models::Teacher { creds: Credentials { username: "Duc".to_string(), password: "l4crypt0C3stR1g0l0".to_string() } });
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