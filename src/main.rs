use common::messages::{Request, Response};
use serde_json::de::from_slice;
use serde_json::ser::to_vec;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // Interact with Master Server
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let request = Request::CreateFile("testfile".to_string());
    let request_data = to_vec(&request).unwrap();
    stream.write(&request_data).unwrap();

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let response: Response = from_slice(&buffer).unwrap();
    println!("{:?}", response);

    // Interact with Chunk Server
    let mut chunk_stream = TcpStream::connect("127.0.0.1:7879").unwrap();
    let chunk_request = Request::WriteChunk("chunk1".to_string(), b"HelloWorld".to_vec());
    let chunk_request_data = to_vec(&chunk_request).unwrap();
    chunk_stream.write(&chunk_request_data).unwrap();

    chunk_stream.read(&mut buffer).unwrap();
    let chunk_response: Response = from_slice(&buffer).unwrap();
    println!("{:?}", chunk_response);
}
