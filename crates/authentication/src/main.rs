extern crate dotenv;
use dotenv::dotenv;
use std::env;


fn hash_data() 
{
    let local_salt = env::var("/home/oj/server/crates/authentication/LOCAL_SALT.env").expect("local salt must be set");
    println!("Local salt is {}",local_salt);
}
use rusqlite::{Connection, Result};

// use argon2::{
//     password_hash::{
//         rand_core::Osrng,
//         PasswordHash, PasswordHasher, PasswordVerifier, SaltString
//     },
//     Argon2
// };

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    password: String,
}

// fn add_user(input_name:String , input_password:String) -> Result<()>
// {
//     let conn = Connection::open("my_database.db")?;

//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS person (
//             id INTEGER PRIMARY KEY,
//             name TEXT NOT NULL,
//             password TEXT NOT NULL
//         )",
//         (),
//     )?;
//     let password = b"hunter42"; // Bad password; don't actually use!
//     let salt = SaltString::generate(&mut SeedableRng);

//     // Argon2 with default params (Argon2id v19)
//     let argon2 = Argon2::default();

//     let me = Person {
//         id: 0,
//         name: input_name,
//         password: input_password,
//     };

//     conn.execute(
//         "INSERT INTO person (name, password) VALUES (?1, ?2)",
//         [&me.name, &me.password],
//     )?;
//     Ok(())
// }

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
    hash_data();

    // let me = Person {
    //     id: 0,
    //     name: "root".to_string(),
    //     password: "notroot".to_string(),
    // };

    // conn.execute(
    //     "INSERT INTO person (name, password) VALUES (?1, ?2)",
    //     [&me.name, &me.password],
    // )?;

    let mut stmt = conn.prepare("SELECT id, name, password FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            password: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
}
