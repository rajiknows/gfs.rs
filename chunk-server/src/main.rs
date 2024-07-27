use common::messages::{Request, Response};
use serde_json::de::from_slice;
use serde_json::ser::to_vec;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct ChunkServer {
    storage: Arc<Mutex<HashMap<String, Vec<u8>>>>, // Chunk ID to data mapping
}

impl ChunkServer {
    fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let request: Request = from_slice(&buffer).unwrap();

        let response = match request {
            Request::WriteChunk(chunk_id, data) => {
                self.storage.lock().unwrap().insert(chunk_id, data);
                Response::ChunkWritten
            }
            _ => unimplemented!(),
        };

        let response_data = to_vec(&response).unwrap();
        stream.write(&response_data).unwrap();
    }

    fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:7879").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let chunk_clone = self.clone();
            std::thread::spawn(move || {
                chunk_clone.handle_client(stream);
            });
        }
    }
}

fn main() {
    let chunk_server = ChunkServer::new();
    chunk_server.run();
}
