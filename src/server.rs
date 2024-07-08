use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

use crate::prelude::*;

fn contenttype_from_extension(extension: &str) -> Option<&str> {
    Some(match extension {
        "html" => "text/html",
        "js" => "application/javascript",
        "wasm" => "application/wasm",
        _ => return None,
    })
}

fn handle_request(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();
    let endpoint =
        &request[0][request[0].find("/").unwrap() + 1..request[0].find("H").unwrap() - 1];
    let ext = endpoint.find(".");
    let extension = if let Some(ext) = ext {
        &endpoint[ext + 1..]
    } else {
        Default::default()
    };

    let contents = match endpoint {
        "entity" => Some(serde_json::to_string(&entity::Entity::new(
            entity::Pos2::new(0.5, 0.0),
            entity::Vel2::new(0.0, 0.0),
            1.0,
        ))?),
        target => fs::read_to_string(format!("src/res/{target}")).ok()
    };
    let content_type = contenttype_from_extension(extension).unwrap_or("text/plain");

    let status_line = match contents {
        Some(_) => "HTTP/1.1 200 OK",
        None => "HTTP/1.1 404 Not Found"
    };
    let contents = contents.unwrap_or("404".to_owned());
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes())?;

    Ok(())
}

pub fn spawn_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    for stream in listener.incoming() {
        handle_request(stream?)?;
    }

    Ok(())
}
