use byte_array::ByteArray;
use uuid::Uuid;

pub struct pdf {
    pub id: Uuid::new_v4(),
    pub name: String,
    pub file_size: usize,
    pub content: ByteArray::new(),
    pub cheksum: String,
}
