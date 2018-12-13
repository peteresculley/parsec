// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct PeerIndex(pub(super) usize);

impl PeerIndex {
    /// `PeerIndex` of ourselves.
    pub const OUR: Self = PeerIndex(0);
}

pub(crate) type PeerIndexSet = BTreeSet<PeerIndex>;
pub(crate) type PeerIndexMap<T> = BTreeMap<PeerIndex, T>;
