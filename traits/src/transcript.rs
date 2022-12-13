use crate::evolvement::Evolvement;
use crate::state::EidState;
use std::fmt::Debug;

pub trait Transcript<E: Evolvement, S: EidState<E>> {
    type StateProvider: EidState<E>
        // require that a transcript state can be created from a client state
        + From<S>
        + Debug;
    /// creates a new log from a trusted [EidState] and a vector of evolvements that happened after the trusted [EidState].
    fn new(trusted_state: Self::StateProvider, log: Vec<E>) -> Self;

    /// Add a new entry on top of the existing [Evolvement]s in the transcript.
    fn add_evolvement(&mut self, evolvement: E);

    /// Returns the trusted state.
    fn trusted_state(&self) -> Self::StateProvider;

    /// Return the [Evolvement]s that happened after the trusted [EidState].
    fn log(&self) -> Vec<E>;
}
