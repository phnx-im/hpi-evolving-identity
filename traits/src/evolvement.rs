pub trait Evolvement {
    fn is_valid_successor(&self, previous: &Self) -> bool;
}