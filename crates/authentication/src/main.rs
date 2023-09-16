extern crate dotenv;
extern crate rand; 

use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use rand::Rng;
use rusqlite::{Connection, Result};
use argon2rs::defaults::{KIB, LANES, PASSES};
use argon2rs::verifier::Encoded;
use argon2rs::{Argon2, Variant};

fn dehash_data(random_salt : String) -> String
{
    String::from("value")
}

fn hash_data(password:String) -> (String,String) 
{
    let mut file = File::open("LOCAL_SALT.env").expect("File Opened Successfuly");
    let mut local_salt = String::new() ; 
    file.read_to_string(&mut local_salt).expect("."); 
    env::set_var("LOCAL_SALT", &local_salt);

    // let random_salt = rand::thread_rng().gen()::<str>().take(32).collect::<String>();
    let random_salt: String = rand::thread_rng()
    .sample_iter(rand::distributions::Alphanumeric)
    .take(32) // Adjust the length of the random string as needed
    .map(char::from)
    .collect();

    println!("Random Salt={:}",random_salt);

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let random_salt_hash = Encoded::new(a2, random_salt.as_bytes(), local_salt.as_bytes(), b"", b"").to_u8();
    let random_salt_hash_storable_encoding = String::from_utf8(random_salt_hash).unwrap();

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let data_hash = Encoded::new(a2, password.as_bytes(), random_salt_hash_storable_encoding.as_bytes(), b"", b"").to_u8();
    let data_hash_storable_encoding = String::from_utf8(data_hash).unwrap();


    let local_salt = env::var("LOCAL_SALT").expect("local salt must be set");
    println!("{}",local_salt);
    (data_hash_storable_encoding,random_salt_hash_storable_encoding)

}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    password: String,
    salt: String
}

fn add_user(input_name:String , input_password:String) -> Result<()>
{
    let conn = Connection::open("my_database.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        (),
    )?;

    let (hashed_password,hashed_password_salt) = hash_data(input_password);
   
    let me = Person {
        id: 0,
        name: input_name,
        password: hashed_password,
        salt : hashed_password_salt

    };

    conn.execute(
        "INSERT INTO person (name, password) VALUES (?1, ?2)",
        [&me.name, &me.password, &me.salt],
    )?;
    Ok(())
}

fn authenticator(id:String , password:String) -> bool
{
    let conn = Connection::open("my_database.db");
    match conn 
    {
        Ok(conn1) => 
        {
            let mut  stmt = conn1.prepare("SELECT id, name, password FROM person");
            match stmt
            {
                Ok(mut stmt1) =>
                {
                    let person_iter = stmt1.query_map([], |row| {
                        Ok(Person {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            password: row.get(2)?,
                            salt: row.get(3)?
                        })
                    });
                    false ;
            
                    for person in person_iter {
                        return true 
                    }
                    false 
                }
                Err(error) => 
            {
                println!("{:}",error);
                false 
            }
            }

            } 
            Err(error) =>
            {
                println!("{:}",error);
                false 
            }
    }
    
}
fn main() -> Result<()> {
    let conn = Connection::open("my_database.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        (),
    )?;

    //
    dotenv().ok();
    let meowmeow = hash_data(String::from("meowmeow"));
    println!("{:?}",meowmeow);

    add_user(String::from("MEOWMEOW"),String::from("MEOWMEOW"));

    let mut stmt = conn.prepare("SELECT id, name, password FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            password: row.get(2)?,
            salt: row.get(3)?
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
}
