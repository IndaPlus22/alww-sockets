use std::{
    cell::RefCell,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use alww_sockets::ThreadPool;

fn main() {
    // let chat = Rc::new(RefCell::new(Chat::default()));
    let chat = Arc::new(Mutex::new(Chat::default()));
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let chat = Arc::clone(&chat);
        pool.execute(|| {
            handle_connection(stream, chat);
        });
    }
}

fn handle_connection(mut stream: TcpStream, chat: Arc<Mutex<Chat>>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    if request_line == "GET / HTTP/1.1" {
    } else {
        let thingy = &request_line
            .lines()
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .rsplit_once('=')
            .unwrap();
        println!("{:?}", thingy.1);
        let message = thingy.1.to_owned();
        let chat_lock = chat.lock().unwrap();
    }

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 200 OK", "hello.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
#[derive(Default, Clone)]
struct Chat {
    log: Vec<String>,
}
impl Chat {
    fn add(mut self, message: String) {
        self.log.push(message);
        self.update()
    }
    fn update(&self) {
        let mut file = std::fs::File::open("hello.html").unwrap();
        let mut first_contents = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Chat!</title>
  </head>
  <body>
    <h1>Chat!</h1>"#;

        let mut second_contents = r#"
    <form action="/chat" method="get">
      <label for="message">Chat:</label>
      <input type="text" id="message" name="message" /><br /><br />
      <input type="submit" value="Submit" />
    </form>
  </body>
</html>
    "#;
        let mut middle_content = String::new();
        for x in &self.log {
            middle_content.push_str(x.as_str());
        }
        let mut complete_string = String::new();
        complete_string.push_str(first_contents);
        complete_string.push_str(middle_content.as_str());
        complete_string.push_str(second_contents);

        file.write_all(complete_string.as_bytes());
    }
}
