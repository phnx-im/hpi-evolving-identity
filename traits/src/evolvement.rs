pub trait Evolvement: Clone {
    fn is_valid_successor(&self, previous: &Self) -> bool;
}
