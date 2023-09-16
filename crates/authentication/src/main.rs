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
    let local_salt = env::var("LOCAL_SALT").expect("LOCAL_SALT must be set");

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let random_salt_hash = Encoded::new(a2, hashed_salt.as_bytes(), local_salt.as_bytes(), b"", b"").to_u8();
    let random_salt_hash_storable_encoding = String::from_utf8(random_salt_hash).unwrap();

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let data_hash = Encoded::new(a2, password.as_bytes(), random_salt_hash_storable_encoding.as_bytes(), b"", b"").to_u8();
    let data_hash_storable_encoding = String::from_utf8(data_hash).unwrap();

    data_hash_storable_encoding

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

    /*
    fn new(argon: Argon2, p: &[u8], s: &[u8], k: &[u8], x: &[u8]) -> Self
    Generates a new hashing session from password, salt, and other byte input. Parameters are:
    argon: An Argon2 struct representative of the desired hash algorithm parameters.
    p: Password input. as bytes 
    s: Salt. as bytes 
    k: An optional secret value. 
    x: Optional, miscellaneous associated data.
    Note that p, s, k, x must conform to the same length constraints dictated by Argon2::hash.
    */

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
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        (),
    )?;

    let (password,password_salt) = hash_data(input_password);
   
    let me = Person {
        id: 0,
        name: input_name,
        password: password,
        salt : password_salt

    };

    conn.execute(
        "INSERT INTO person (name, password) VALUES (?1, ?2)",
        [&me.name, &me.password, &me.salt],
    )?;
    Ok(())
}

fn authenticator(input_name:String , password:String) -> bool
{
    let mut boolvar = false;
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
                       Person{
                        id = row.get(0)?,
                       }
                        // match row.get(2) 
                        // {
                        //     Ok(row_name) =>
                        //     {
                        //         match row.get(2)
                        //         {
                        //             Ok(row_password) =>
                        //             {
                        //                 if (row_name == input_name ) && ( row_password == dehash_data(password, row.get(3)?)){
                        //                         boolvar = true;  
                        //                 }
                        //                 Ok(())
                        //             }
                        //             Err(error) => Err(error)
                        //         }
                        //     }
                        //     Err(error) => Err(error)   
                        // }
                    });
                }
                Err(error) => println!("{:}",error)
            }
        }
        Err(error) => println!("Connection failed")
    }
        boolvar
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

    let mut stmt = conn.prepare("SELECT id, name, password FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            password: row.get(2)?,
            salt: row.get(3)?
        })
    })?;
    println!("1");
    // add_user(String::from("oj"),String::from("oj"));
    let (test_password, test_salt) =hash_data(String::from("oj")); 

    println!("hash data function \n\n {:}\n\n {:}",test_password,test_salt);
    
    println!("\n\n2");

    println!("final data\n\n {:}",dehash_data(String::from("oj"), test_salt));
    
    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }



    Ok(())
}
