use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::state::EidState;

pub trait EidTranscript: Default {
    type EvolvementProvider: Evolvement;
    type MemberProvider: Member;
    type StateProvider: EidState<
        EvolvementProvider = Self::EvolvementProvider,
        MemberProvider = Self::MemberProvider,
    >;
    /// creates a new log from a trusted [EidState] and a vector of evolvements that happened after the trusted [EidState].
    fn new(trusted_state: Self::StateProvider, log: Vec<Self::EvolvementProvider>) -> Self;

    /// Add a new entry on top of the existing [Evolvement]s in the transcript.
    fn add_evolvement(&mut self, evolvement: Self::EvolvementProvider);

    /// Returns the trusted state.
    fn trusted_state(&self) -> Self::StateProvider;

    /// Return the [Evolvement]s that happened after the trusted [EidState].
    fn log(&self) -> Vec<Self::EvolvementProvider>;
}
