use postgres::{Client, NoTls, Error as PostgresError};
use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write}; // fixed import

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>, // fixed: Option not Optional
    name: String,
    email: String,
    password: String,
}

// Correctly declared response constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR_RESPONSE: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";


fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    if let Err(e) = set_database() {
        println!("Error: {}", e);
        return;
    }

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server is running on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}

fn get_id(request: &str) -> Option<i32> {
    request.split('/').nth(2)?
        .split_whitespace().next()?
        .parse::<i32>().ok()
}

fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    if stream.read(&mut buffer).is_ok() {
        request.push_str(&String::from_utf8_lossy(&buffer));
        let (status_line, content) = match &*request {
            r if r.starts_with("POST /users") => handle_post_request(r),
            r if r.starts_with("GET /users/") => handle_get_request(r),
            r if r.starts_with("GET /users") => handle_get_all_users(),
            r if r.starts_with("PUT /users/") => handle_put_request(r),
            r if r.starts_with("DELETE /users/") => handle_delete_request(r),
            _ => (NOT_FOUND_RESPONSE.to_string(), "Not Found".to_string()),
        };
        stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
    }
}

fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(request), Client::connect("DATABASE_URL", NoTls)) {
        (Ok(user), Ok(mut client)) => {
            let _ = client.execute(
                "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
                &[&user.name, &user.email, &user.password],
            );
            (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Internal Server Error".to_string()),
    }
}

fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(request), Client::connect("DATABASE_URL", NoTls)) {
        (Some(id), Ok(mut client)) => {
            match client.query_opt("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(Some(row)) => {
                    let user = User {
                        id: Some(row.get(0)),
                        name: row.get(1),
                        email: row.get(2),
                        password: row.get(3),
                    };
                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (NOT_FOUND_RESPONSE.to_string(), "User Not Found".to_string()),
            }
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Internal Server Error".to_string()),
    }
}

fn handle_get_all_users() -> (String, String) {
    match Client::connect("DATABASE_URL", NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();
            for row in client.query("SELECT * FROM users", &[]).unwrap() {
                users.push(User {
                    id: Some(row.get(0)),
                    name: row.get(1),
                    email: row.get(2),
                    password: row.get(3),
                });
            }
            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Internal Server Error".to_string()),
    }
}

fn handle_put_request(request: &str) -> (String, String) {
    match (get_id(request), get_user_request_body(request), Client::connect("DATABASE_URL", NoTls)) {
        (Some(id), Ok(user), Ok(mut client)) => {
            let _ = client.execute(
                "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4",
                &[&user.name, &user.email, &user.password, &id],
            );
            (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Internal Server Error".to_string()),
    }
}

fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(request), Client::connect("DATABASE_URL", NoTls)) {
        (Some(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();
            if rows_affected == 0 {
                (NOT_FOUND_RESPONSE.to_string(), "User Not Found".to_string())
            } else {
                (OK_RESPONSE.to_string(), "User Deleted".to_string())
            }
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Internal Server Error".to_string()),
    }
}

fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect("DATABASE_URL", NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL
        )
    ",
    )?;
    Ok(())
}
