use crate::HashType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LtlHash(pub(crate) HashType);
