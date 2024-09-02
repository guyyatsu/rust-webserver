use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {

    // TODO: Allow artuments for setting custom port.
    //let args: Vec<String> = env::args().collect();

    println!("Setting up server on 13337.");
    let listener = TcpListener::bind("127.0.0.1:13337").unwrap();

    for stream in listener.incoming() {
        handle_connection( stream.unwrap() );
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line= buf_reader.lines().next().unwrap().unwrap();


    let (status_line, filename) =
    
    // Route to main home page.
    if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src\\HTML\\index.html")
    }
    
    // Route to graphic design disipline.
    else if request_line == "GET /graphics HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src\\HTML\\graphics.html")
    }

    // Route to music production discipline. 
    else if request_line == "GET /musics HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src\\HTML\\musics.html")
    }

    // Route to street journalism discipline.
    else if request_line == "GET /journalism" {
        ("HTTP/1.1 200 OK", "src\\HTML\\journalism.html")
    }

    // If all else fails, return a 404 error. 
    else {
        ("http/1.1 400 NOT FOUND", "src\\HTML\\404.html")
    };
        
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();

}
