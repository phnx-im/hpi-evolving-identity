pub trait Member {
    fn new() -> Self;
    fn get_pk(&self) -> Vec<u8>;
}
