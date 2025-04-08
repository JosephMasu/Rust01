use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::env;
use sdt::net::TcpListener;
use std::net::TcpStream;
use std::{Read, Write};


#[macro_use]
extern crate serde_derive;

// Model: User struct with id, name, email and password
# [derive(Serialize, Deserialize)]
struct User{
    id: Optional<i32>,
    name: String,
    email: String,
    password: String,
}

//DATABASE_URL

const DATABASE_URL: &str = env!("DATABASE_URL");

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR_RESPONSE: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";


fn main() {
// Set Database

if let Err(e) = set_database() {
    println!("Error: {}", e);
    return;
}

//start server
let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
println!("Server is running on port 8080");

//handle client connection
for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            handle_client(stream);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
}

// get id function
fn get_id(request: String) -> Option<i32> {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default();
}

// deserialize user from request
fn deserialize_user(request: String) -> Result<User, serde_json::Error> {
    serde_json::from_str(&request.split("\r\n\r\n").last().unwrap_or_default())
}

// handle client function
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();
    
    match stream.read(&mut buffer) {
        Ok(_) => {
            request.push_str(String::from_utf8_lossy(&buffer).as_ref());
            let(status_line, content) = match &*request {
            r if r.starts_with("POST /users") => handle_post_request(request),
            r if r.starts_with("GET /users") => handle_get_request(request),
            r if r.starts_with("GET /users/") => handle_get_allusers(request),
            r if r.starts_with("PUT /users/") => handle_put_request(request),
            r if r.starts_with("DELETE /users/") => handle_delete_request(request),
            _ => (NOT_FOUND_RESPONSE.to_string(), "Not Found".to_string())
        };
        stream.write(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

//handle post request

fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(&request), client::connect(DATABASE_URL, NoTls)) {

        (Ok(user), Ok(mut client)) => {
            client.execute(
                "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
                &[&user.name, &user.email, &user.password]
            ).unwrap();
            (OK_RESPONSE, serde_json::to_string(&user).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE, "Internal Server Error".to_string())
    }
}

//handle get request

fn handle_get_request(request: &str) -> (&String, String) {
    match (get_id(&request).parse::<i32>(), client::connect(DATABASE_URL, NoTls)){
        (Ok(id), Ok(mut client)) => {
            match client.query("SELECT * FROM users WHERE id = $1", &[&id]){
                Ok(row) =>{
                    let mut user = User{
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                        password: row.get(3),  
                    };

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _=>(NOT_FOUND.to_string(), "User Not Found".to_string())

            }
        }
        _=>(INTERNAL_SERVER_ERROR.to_string(), "Internal Server Error".to_string())
    }
}

// handle get all users request

fn handle_get_allusers(request: &str) -> (String, String) {
    match client::connect(DATABASE_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();

            for row in client.query("SELECT * FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                    password: row.get(3),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
            }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Internal Server Error".to_string())
    }    
}

// handle put request

fn handle_put_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        deserialize_user(request),
        client::connect(DATABASE_URL, NoTls)
        ) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client.execute(
                "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4",
                &[&user.name, &user.email, &user.password, &id]
            ).unwrap();
            (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
        }
        _=>(INTERNAL_SERVER_ERROR.to_string(), "Internal Server Error".to_string())
    }
    
}

//handle delete request
fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), client::connect(DATABASE_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();
            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User Not Found".to_string());
            }
            
            (OK_RESPONSE.to_string(), "User Deleted".to_string())
        }
        _=>(INTERNAL_SERVER_ERROR.to_string(), "Internal Server Error".to_string())
    }
}

//set database function
fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DATABASE_URL, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL
        )
        "
    )?;
    Ok(())
}