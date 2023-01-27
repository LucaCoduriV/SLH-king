extern crate core;

mod database;
mod models;
mod auth;

use log::error;
use read_input::prelude::*;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use once_cell::sync::Lazy;
use dotenv::dotenv;
use std::env;
use crate::auth::{AuthError, login_as_student, login_as_teacher};

const DATABASE_FILE: &str = "db.json";

static DB_INSTANCE: Lazy<database::DB> = Lazy::new(|| {
    database::DB::from_file(DATABASE_FILE, SECRET.as_str(), NONCE.as_str()).unwrap_or_else(|_| {
        eprintln!("Error while reading db file, using empty db");
        database::DB::from_env()
    })
});

static SECRET: Lazy<String> = Lazy::new(|| {
    env::var("SECRET").unwrap()
});

static NONCE: Lazy<String> = Lazy::new(|| {
    env::var("NONCE").unwrap()
});

enum AccountType {
    Teacher(String),
    Student(String),
}

fn welcome() {
    println!("Welcome to KING: KING Is Not GAPS");
}

fn ask_creds() -> Result<AccountType, AuthError> {
    let choice = input().inside(['t', 's']).msg("Are you a teacher or a student ? t/s: ").get();
    let is_teacher = choice == 't';

    println!("Enter your username:");
    let username: String = input().get();
    println!("Enter your password:");
    let password: String = input().get(); // cacher le mot de passe est mieux, mais pas obligé pour ce labo.

    match is_teacher {
        true => { login_as_teacher(&DB_INSTANCE, username.as_str(), password.as_str()).map(|_| AccountType::Teacher(username)) }
        false => { login_as_student(&DB_INSTANCE, username.as_str(), password.as_str()).map(|_| AccountType::Student(username)) }
    }
}

fn menu(account_type: &AccountType) {
    while let Ok(_) = match account_type {
        AccountType::Teacher(_) => teacher_action(),
        AccountType::Student(username) => student_action(username),
    }{}
}

fn student_action(username: &String) -> Result<(), ()> {
    println!("*****\n1: See your grades\n2: Logout\n0: Quit");
    let choice = input().inside(0..=2).msg("Enter Your choice: ").get();
    match choice {
        1 => Ok(show_grades(username.as_str())),
        2 => Err(()),
        0 => Ok(quit()),
        _ => Ok(error!("impossible choice")),
    }
}

fn teacher_action() -> Result<(), ()> {
    println!("*****\n1: See grades of student\n2: Enter grades\n3: Logout\n0: Quit");
    let choice = input().inside(0..=3).msg("Enter Your choice: ").get();
    match choice {
        1 => {
            let student_username: String = input().msg("Enter the name of the user of which you want to see the grades:").get();
            return Ok(show_grades(student_username.as_str()))
        },
        2 => Ok(enter_grade()),
        3 => Err(()),
        0 => Ok(quit()),
        _ => Ok(error!("impossible choice")),
    }
}

fn show_grades(username: &str) {
    let db = DB_INSTANCE.students.lock().unwrap();
    match db.get(username) {
        Some(student) => {
            println!("Here are the grades of user {}", username);
            println!("{:?}", student.grades);
            println!(
                "The average is {}",
                (student.grades.iter().sum::<f32>()) / ((*student.grades).len() as f32)
            );
        }
        None => println!("User not in system"),
    };
}

fn enter_grade() {
    println!("What is the name of the student?");
    let name: String = input().get();
    println!("What is the new grade of the student?");
    let grade: f32 = input().add_test(|x| *x >= 0.0 && *x <= 6.0).get();
    let mut map = DB_INSTANCE.students.lock().unwrap();
    match map.get_mut(&name) {
        Some(v) => v.grades.push(grade),
        None => {
            eprintln!("user does not exist");
        }
    };
}

fn quit() {
    println!("Saving database!");
    if let Ok(_) = DB_INSTANCE.save(DATABASE_FILE, SECRET.as_str(), NONCE.as_str()) {
        std::process::exit(0);
    }
    std::process::exit(1);
}

fn main() {
    dotenv().ok();
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    ).unwrap();

    welcome();
    loop {
        match ask_creds() {
            Err(_) => eprintln!("Username or password is wrong!"),
            Ok(account_type) => menu(&account_type),
        }
    }
}
