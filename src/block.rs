// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use crate::error::Error;
use crate::id::{Proof, PublicId};
use crate::network_event::NetworkEvent;
use crate::observation::Observation;
use crate::vote::Vote;
use std::collections::{BTreeMap, BTreeSet};

/// A struct representing a collection of votes by peers for an `Observation`.
#[serde(bound = "")]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Block<T: NetworkEvent, P: PublicId> {
    payload: Observation<T, P>,
    proofs: BTreeSet<Proof<P>>,
}

impl<T: NetworkEvent, P: PublicId> Block<T, P> {
    /// Creates a `Block` from `votes`.
    pub fn new(votes: &BTreeMap<P, Vote<T, P>>) -> Result<Self, Error> {
        let payload = if let Some(vote) = votes.values().next() {
            vote.payload().clone()
        } else {
            return Err(Error::MissingVotes);
        };

        let proofs: Result<BTreeSet<_>, _> = votes
            .iter()
            .map(|(public_id, vote)| {
                if *vote.payload() == payload {
                    vote.create_proof(public_id)
                } else {
                    Err(Error::MismatchedPayload)
                }
            })
            .collect();
        let proofs = proofs?;

        Ok(Self { payload, proofs })
    }

    /// Returns the payload of this block.
    pub fn payload(&self) -> &Observation<T, P> {
        &self.payload
    }

    /// Returns the proofs of this block.
    pub fn proofs(&self) -> &BTreeSet<Proof<P>> {
        &self.proofs
    }

    /// Converts `vote` to a `Proof` and attempts to add it to the block.  Returns an error if
    /// `vote` is invalid (i.e. signature check fails or the `vote` is for a different network
    /// event), `Ok(true)` if the `Proof` wasn't previously held in this `Block`, or `Ok(false)` if
    /// it was previously held.
    pub fn add_vote(&mut self, peer_id: &P, vote: &Vote<T, P>) -> Result<bool, Error> {
        if &self.payload != vote.payload() {
            return Err(Error::MismatchedPayload);
        }
        let proof = vote.create_proof(peer_id)?;
        Ok(self.proofs.insert(proof))
    }
}
