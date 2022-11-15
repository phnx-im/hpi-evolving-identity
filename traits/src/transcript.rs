
use crate::evolvement::Evolvement;
use crate::state::EidState;

pub trait Transcript {
    fn new(trusted_state: impl EidState, log: Vec<impl Evolvement>) -> Self;
    fn verify() -> bool;
}