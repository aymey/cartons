use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread::{self, sleep}, time::{self, Duration}
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

fn simulate(simulation: Arc<Mutex<simulation::Simulation>>) {
    let mut last = time::Instant::now();
    loop {
        thread::sleep(Duration::from_millis(3));
        let now = time::Instant::now();
        let delta = now.duration_since(last);
        last = now;

        {
            let mut simulation = simulation.lock().unwrap();
            simulation.step(delta.as_secs_f32());
        }
    }
}

fn handle_request(
    mut stream: TcpStream,
    simulation: Arc<Mutex<simulation::Simulation>>,
) -> std::io::Result<()> {
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
        "entity" => Some(serde_json::to_string(&simulation.entities[0])?),
        target => fs::read_to_string(format!("src/res/{target}")).ok(),
    };
    let content_type = contenttype_from_extension(extension).unwrap_or("text/plain");

    let status_line = match contents {
        Some(_) => "HTTP/1.1 200 OK",
        None => "HTTP/1.1 404 Not Found",
    };
    let contents = contents.unwrap_or("404".to_owned());
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes())?;

    Ok(())
}

pub fn spawn_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    let mut simulation_ctx = simulation::Simulation::default();
    let ent = entity::Entity::new(entity::Pos::new(0.5, 0.5), entity::Vel::new(0.0, 0.0), 5.0);
    simulation_ctx.add(ent);
    let mutex = Arc::new(Mutex::new(simulation_ctx));
    let clone = Arc::clone(&mutex);

    std::thread::scope(|s| {
        s.spawn(move || simulate(mutex));

        for stream in listener.incoming() {
            handle_request(stream.unwrap(), clone).unwrap();
        }
    });

    Ok(())
}
