use std::{
    net::TcpListener,
    fs,
    io::prelude::*,
    net::TcpStream,
    time::Duration,
    thread
};

mod thread_pool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = thread_pool::ThreadPool::build(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });

        println!("Shutting down.");
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    return_response(stream, status_line, file_name);
}

fn return_response(mut stream: TcpStream, status_line: &str, file_name: &str) {
    let status_line = status_line;
    let contents = fs::read_to_string(file_name).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


