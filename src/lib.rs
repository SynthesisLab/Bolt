pub mod algos;
pub(crate) mod bool;
pub(crate) mod cache;
pub mod formula;
pub mod ltl;
pub(crate) mod ops;
pub mod traits;

/// Hash type of [`ltl::LtlFormula`] and [`bool::BoolFormula`], parametric for easier configuration.
type HashType = u64;
