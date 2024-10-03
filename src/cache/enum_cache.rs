use crate::{formula::Formula, traits::Hashed};

use super::FormulaCache;

pub(crate) trait EnumFormulaCache<Char>:
    FormulaCache<Char> + IntoIterator<Item = Formula<Char>>
where
    Char: Hashed,
{
    type CacheLine<'a>: EnumFormulaCacheLine<Char>
    where
        Char: 'a,
        Self: 'a;

    /// Creates a new chache line for formulas of size `size`,
    /// and return an iterator over formulas of size `size-1`
    /// and over pairs of formulas whose size sums up to `size-1`.
    fn new_line_and_iter_size<'a>(
        &'a mut self,
        size: usize,
    ) -> (
        impl Iterator<Item = &'a Formula<Char>>,
        impl Iterator<Item = (&'a Formula<Char>, &'a Formula<Char>)>,
        Self::CacheLine<'a>,
    )
    where
        Char: 'a;

    fn new_line<'a>(&'a mut self, size: usize) -> Self::CacheLine<'a>
    where
        Char: 'a;

    fn iter_size<'a>(&'a self, size: usize) -> impl Iterator<Item = &'a Formula<Char>>
    where
        Char: 'a;

    fn nb_lines(&self) -> usize;
}

pub(crate) trait EnumFormulaCacheLine<Char>
where
    Char: Hashed,
{
    fn push(&mut self, item: Formula<Char>) -> bool;
}
