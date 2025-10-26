use rusty::{Request, Response, Rusty};
use std::thread;
use std::time::Duration;

#[test]
fn test_server_response() {
    thread::spawn(|| {
        let mut rusty = Rusty::new();
        rusty.get("/ping", |_req: &Request| Response::new(200, "pong"));
        rusty.listen(7878);
    });

    thread::sleep(Duration::from_millis(200));

    let response = reqwest::blocking::get("http://127.0.0.1:7878/ping")
        .unwrap()
        .text()
        .unwrap();

    assert_eq!(response, "pong");
}

// #[test]
// fn test_handle_connection_logic() {
//     use std::io::Write;
//     use std::net::{TcpListener, TcpStream};
//
//     let listener = TcpListener::bind("127.0.0.1:0").unwrap();
//     let port = listener.local_addr().unwrap().port();
//
//     thread::spawn(move || {
//         let mut rusty = Rusty::new();
//         rusty.get("/", |_req| Response::new(200, "ok"));
//         rusty.listen(port);
//     });
//
//     thread::sleep(Duration::from_millis(200));
//     let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
//     stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();
//
//     let mut buffer = String::new();
//     std::io::Read::read_to_string(&mut stream, &mut buffer).unwrap();
//
//     assert!(buffer.contains("ok"));
// }
