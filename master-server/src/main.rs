use common::messages::{Request, Response};
use serde_json::de::from_slice;
use serde_json::ser::to_vec;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct MasterServer {
    metadata: Arc<Mutex<HashMap<String, Vec<String>>>>, // File to chunks mapping
}

impl MasterServer {
    fn new() -> Self {
        Self {
            metadata: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let request: Request = from_slice(&buffer).unwrap();

        let response = match request {
            Request::CreateFile(filename) => {
                self.metadata.lock().unwrap().insert(filename, vec![]);
                Response::FileCreated
            }
            _ => unimplemented!(),
        };

        let response_data = to_vec(&response).unwrap();
        stream.write(&response_data).unwrap();
    }

    fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let master_clone = self.clone();
            std::thread::spawn(move || {
                master_clone.handle_client(stream);
            });
        }
    }
}

fn main() {
    let master = MasterServer::new();
    master.run();
}
