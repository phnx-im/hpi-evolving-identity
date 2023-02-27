# Evolving Identity using MLS

The idea of using MLS for the implementation of an EID is to leverage its properties for the EID, namely distributed,
asynchronous, scalable and secure group management.
The EID is built as a logical layer on top of MLS, using the _openMLS_ library.

An EID _Client_ manages its state through an _openMLS_ Group.
Since an MLS Group changes its state by applying _Commits_, the _Evolvement_ for this implementation wraps such a
Commit.
The EID functions `add()`, `update()`, `remove()` call the corresponding functions on the MLS group, creating the
Commit.
Then the _Commit_ is wrapped as an _Evolvement_.

Cross-signing is implemented by creating an update Commit.
This makes use of the fact that _openMLS_ members that have updated at least once can easily be distinguished
from members that have never been updated in context of MLS.

Evolving is done by unwrapping the _Commit_ from the _Evolvement_ and processing it with the MLS Group.
The MLS Group takes care of validation and state transition.

The MLS Group and thus the EID _Client_ state rely on private key material, to prevent non-members from creating Commits
or _Evolvements_, respectively.
This means that creating a public _Transcript_ directly from a _Client_ state is not possible without compromising the
Client.

To address that problem, openMLS has recently introduced separation of public and secret states which allows to extract
a public group from any MLS Group.
We use this public group as the _Transcripts_ state.
As the _Transcript_ requires, the public group is capable of processing any _Commit_ of the original MLS group but not
creating new Commits.
To instantiate a _Transcript_, a _Client_ exports the information that is needed to create a public group.
This can then be sent to the other party which wants to initialize the _Transcript_.
Upon _Transcript_ creation, this information is verified and used to create a public group.