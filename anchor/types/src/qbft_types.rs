use derive_more::{Add, Deref, From};
/// A collection of types used by the QBFT modules
use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;

/// This represents an individual round, these change on regular time intervals
#[derive(Clone, Copy, Debug, Deref, Default, Add, PartialEq, Eq, Hash, PartialOrd)]
pub struct Round(usize);

impl Round {
    /// Returns the next round
    pub fn next(&self) -> Round {
        Round(self.0 + 1)
    }

    /// Sets the current round
    pub fn set(&mut self, round: Round) {
        *self = round;
    }
}

/// The operator that is participating in the consensus instance.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, From, Deref)]
pub struct OperatorId(usize);

/// The instance height behaves like an "ID" for the QBFT instance. It is used to uniquely identify
/// different instances, that have the same operator id.
#[derive(Clone, Copy, Debug, Default)]
pub struct InstanceHeight(usize);

impl Deref for InstanceHeight {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
