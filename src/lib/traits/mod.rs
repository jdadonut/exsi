
pub trait IntoRaw {
    fn into_raw(&self) -> Box<&[u8]>;
}
pub trait ReadFromAt {
    fn read_bytes_at(&mut self, offset: u64, bytes: usize) -> Vec<u8>;
}