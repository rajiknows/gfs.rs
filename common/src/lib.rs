pub mod messages {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub enum Request {
        CreateFile(String),
        WriteChunk(String, Vec<u8>),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Response {
        FileCreated,
        ChunkWritten,
    }
}
