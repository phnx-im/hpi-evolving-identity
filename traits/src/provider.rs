use crate::client::EidClient;
use crate::transcript::EidTranscript;

trait EidProvider {
    type ClientProvider: EidClient;
    type TranscriptProvider: EidTranscript<
        EvolvementProvider = <Self::ClientProvider as EidClient>::EvolvementProvider,
        MemberProvider = <Self::ClientProvider as EidClient>::MemberProvider,
    >;
}
