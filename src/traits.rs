use std::{fmt::Debug, hash::Hash};

pub(crate) trait EqTarget {
    type TargetType: ?Sized;
    fn eq_target(&self, target: &Self::TargetType) -> bool;
}

/// A type that can be [`Hash`]ed and return its hash.
///
/// This is different from [`Hash`], as [`Hashed`] gives an opportunity
/// to store the hash and avoid recomputing it at every call.
pub trait Hashed {
    type HashType: Eq + Hash + Clone + Copy + Debug;
    fn hashed(&self) -> Self::HashType;
}
