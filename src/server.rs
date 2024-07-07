use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::prelude::*;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();
    let endpoint =
        &request[0][request[0].find("/").unwrap() + 1..request[0].find("H").unwrap() - 1];

    let status_line = "HTTP/1.1 200 OK";

    let (contents, content_type) = match endpoint {
        "entity" => (serde_json::to_string(&entity::Entity::new(
            entity::Pos2::new(0.5, 0.0),
            entity::Vel2::new(0.0, 0.0),
            1.0,
        ))?, "application/json"),
        "index.js" => (fs::read_to_string("src/res/index.js")?, "application/javascript"),
        _ => (fs::read_to_string("src/res/index.html")?, "text/html"),
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes())?;

    Ok(())
}

pub fn spawn_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
