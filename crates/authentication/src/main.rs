extern crate dotenv;
extern crate rand; 
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Read;
use rand::Rng;
use rusqlite::{Connection, Result};
use argon2rs::defaults::{KIB, LANES, PASSES};
use argon2rs::verifier::Encoded;
use argon2rs::{Argon2, Variant};

fn dehash_data(password : String , hashed_salt : String ) -> String
{
    let mut file = File::open("LOCAL_SALT.env").expect("File Opened Successfuly");
    let mut local_salt = String::new() ; 
    file.read_to_string(&mut local_salt).expect("."); 
    env::set_var("LOCAL_SALT", &local_salt);
    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let random_salt_hash = Encoded::new(a2, hashed_salt.as_bytes(), local_salt.as_bytes(), b"", b"").to_u8();
    let random_salt_hash_storable_encoding = String::from_utf8(random_salt_hash).unwrap();

    //random salt is not generated in this case but instead is fetched from the database 

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let data_hash = Encoded::new(a2, password.as_bytes(), random_salt_hash_storable_encoding.as_bytes(), b"", b"").to_u8();
    let data_hash_storable_encoding = String::from_utf8(data_hash).unwrap();

    data_hash_storable_encoding

}

fn hash_data(password:&String) -> (String,String) 
{
    let mut file = File::open("LOCAL_SALT.env").expect("File Opened Successfuly");
    let mut local_salt = String::new() ; 
    file.read_to_string(&mut local_salt).expect("."); 
    env::set_var("LOCAL_SALT", &local_salt);
    let random_salt: String = rand::thread_rng()
    .sample_iter(rand::distributions::Alphanumeric)
    .take(32) // Adjust the length of the random string as needed
    .map(char::from)
    .collect();

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let random_salt_hash = Encoded::new(a2, random_salt.as_bytes(), local_salt.as_bytes(), b"", b"").to_u8();
    let random_salt_hash_storable_encoding = String::from_utf8(random_salt_hash).unwrap();

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let data_hash = Encoded::new(a2, password.as_bytes(), random_salt_hash_storable_encoding.as_bytes(), b"", b"").to_u8();
    let data_hash_storable_encoding = String::from_utf8(data_hash).unwrap();


    let local_salt = env::var("LOCAL_SALT").expect("local salt must be set");
    // println!("hash data function \n\n {:}\n\n {:}",data_hash_storable_encoding,random_salt_hash_storable_encoding);
    (data_hash_storable_encoding,random_salt)

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
        "CREATE TABLE IF NOT EXISTS person1 (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL,
            salt TEXT NOT NULL
        )",
        (),
    )?;

    let (password,password_salt) = hash_data(&input_password);
   
    let me = Person {
        id: 0,
        name: input_name,
        password: password,
        salt : password_salt

    };

    conn.execute(
        "INSERT INTO person1 (name, password, salt) VALUES (?1, ?2, ?3)",
        [&me.name, &me.password, &me.salt],
    )?;
    Ok(())
}

fn authenticator(input_name:String,input_password:String) -> Result<bool>
{
    let mut boolvar = false;
    let conn = Connection::open("my_database.db");
    match conn 
    {
        Ok(conn1) => 
        {
            let mut  stmt = conn1.prepare("SELECT id, name, password, salt FROM person1");
            match stmt
            {
                Ok(mut stmt1) =>
                {   
                    let rows = stmt1.query_map([], |row|
                    {
                       Ok(
                        Person
                       {
                        id : row.get(0)?,
                        name : row.get(1)?,
                        password : row.get(2)?,
                        salt : row.get(3)?,
                       }
                       )
                    })?;
                    for row in rows 
                    {
                        let row = row.as_ref().unwrap();
                        if input_name == row.name
                        {       
                            let meow_data = dehash_data(input_password.clone(),row.salt.clone());
                            if row.password == meow_data  {
                                boolvar = true ;
                                break;
                            }
                        }
                    }

                    
                }
                Err(error) => println!("{:}",error)
            }
        }
        Err(error) => println!("Connection failed")
    }
       Ok(boolvar)
}

    

fn main() -> Result<()> {
    let conn = Connection::open("my_database.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person1 (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL,
            salt TEXT NOT NULL
        )",
        (),
    )?;
    dotenv().ok();
    // add_user(String::from("Jauhar"), String::from("meow"));
    authenticator(String::from("Jauhar"),String::from("meow"));


    Ok(())
}
