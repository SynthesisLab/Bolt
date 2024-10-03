//! Cache for use with the [Semantic Enumeration algorithm](crate::algos::enumeration)
pub(crate) mod enum_cache;

pub(crate) use enum_cache::{EnumFormulaCache, EnumFormulaCacheLine};

use super::{formula::Formula, traits::Hashed};

/// Trait for structs that can store a set of formulas,
/// and retrieve them by hash.
pub(crate) trait FormulaCache<Char>
where
    Char: Hashed,
{
    fn len(&self) -> usize;
    fn get(&self, hash: &Char::HashType) -> Option<&Formula<Char>>;
}
