## EID Dummy Implementation

The Dummy implementation is a merely a proof of concept.
It does not use any cryptography and only ensures correctness in scope of generic tests.
The state of Dummy Clients and Dummy _Transcripts_ is determined by the current list of members and an _Evolvement_
counter.
An _Evolvement_ consists of the proposed new member list and the incremented _Evolvement_ count.
A member consists of an identifier and an indicator whether they cross-signed their own membership.
Client and _Transcript_ will apply an _Evolvement_ by validating the _Evolvement_ count and
then setting the list of members of the _Evolvement_ as their new state.
This means that the _Client_ state and _Transcript_ state are of the same type and
creating a _Transcript_ is done by transferring the member list from a Client.

This implementation is provided by the library, however this book mainly deals with the MLS EID implementation
which in contrast to this implementation actually meets security requirements.