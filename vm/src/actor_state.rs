// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::TokenAmount;
use cid::Cid;
use num_bigint::biguint_ser::{BigUintDe, BigUintSer};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Identifier for Actors, includes builtin and initialized actors
pub type ActorID = u64;

/// State of all actor implementations
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ActorState {
    pub code: Cid,
    pub state: Cid,
    pub sequence: u64,
    pub balance: TokenAmount,
}

impl Serialize for ActorState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (
            &self.code,
            &self.state,
            &self.sequence,
            BigUintSer(&self.balance),
        )
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ActorState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (code, state, sequence, BigUintDe(balance)) = Deserialize::deserialize(deserializer)?;
        Ok(ActorState {
            code,
            state,
            sequence,
            balance,
        })
    }
}

impl ActorState {
    /// Constructor for actor state
    pub fn new(code: Cid, state: Cid, balance: TokenAmount, sequence: u64) -> Self {
        Self {
            code,
            state,
            balance,
            sequence,
        }
    }
}