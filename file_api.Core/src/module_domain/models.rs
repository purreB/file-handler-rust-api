use uuid::Uuid;

#[derive(Debug)]
pub struct Pdf {
    pub id: Uuid,
    pub name: String,
    pub file_size: usize,
    pub content: Vec<u8>,
    pub checksum: String,
}

impl Pdf {
    pub fn new(
        id: Uuid,
        name: String,
        file_size: usize,
        content: Vec<u8>,
        checksum: String,
    ) -> Self {
        Self {
            id,
            name,
            file_size,
            content,
            checksum,
        }
    }
}
