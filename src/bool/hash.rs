//! Hash of boolean functions

use crate::HashType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolHash(pub(crate) HashType);
