use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("src/res/index.html")?;
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");


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
