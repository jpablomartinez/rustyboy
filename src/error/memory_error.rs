#[derive(Debug)]
pub enum MemoryError {
    InvalidAddress(u16)
}