use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::state::EidState;

pub trait Transcript<E: Evolvement, M: Member> {
    type StateProvider: EidState<E, M>;
    /// creates a new log from a trusted [EidState] and a vector of evolvements that happened after the trusted [EidState].
    fn new(trusted_state: Self::StateProvider, log: Vec<E>) -> Self;

    /// Add a new entry on top of the existing [Evolvement]s in the transcript.
    fn add_evolvement(&mut self, evolvement: E);

    /// Returns the trusted state.
    fn trusted_state(&self) -> Self::StateProvider;

    /// Return the [Evolvement]s that happened after the trusted [EidState].
    fn log(&self) -> Vec<E>;
}
