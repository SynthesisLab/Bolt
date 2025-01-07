//! Characteristic sequence of an LTL formula
//! and related operators.
use std::{
    fmt::{Debug, Display},
    ops::{BitAnd, BitOr, Not},
};

/// Characteristic sequence of an LTL formula on a trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharSeq {
    head: u64,
    cycle: u64,
    head_len: usize,
    cycle_len: usize,
}

fn restrict_to_first_k_bits(x: u64, k: usize) -> u64 {
    if k < 64 {
        x & ((1u64 << k) - 1)
    } else {
        x
    }
}

fn all_true_u64(k: usize) -> u64 {
    if k < 64 {
        (1u64 << k) - 1
    } else {
        !0
    }
}

impl Not for CharSeq {
    type Output = Self;

    fn not(self) -> Self::Output {
        let CharSeq {
            head,
            cycle,
            head_len,
            cycle_len,
        } = self;
        let not_h = restrict_to_first_k_bits(head.not(), head_len);
        let not_c = restrict_to_first_k_bits(cycle.not(), cycle_len);
        CharSeq {
            head: not_h,
            cycle: not_c,
            head_len,
            cycle_len,
        }
    }
}

impl BitOr for CharSeq {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let CharSeq {
            head: h1,
            cycle: c1,
            head_len,
            cycle_len,
        } = self;
        let CharSeq {
            head: h2,
            cycle: c2,
            head_len: hl2,
            cycle_len: cl2,
        } = rhs;
        assert_eq!(head_len, hl2);
        assert_eq!(cycle_len, cl2);
        CharSeq {
            head: h1 | h2,
            cycle: c1 | c2,
            head_len,
            cycle_len,
        }
    }
}

impl BitAnd for CharSeq {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let CharSeq {
            head: h1,
            cycle: c1,
            head_len,
            cycle_len,
        } = self;
        let CharSeq {
            head: h2,
            cycle: c2,
            head_len: hl2,
            cycle_len: cl2,
        } = rhs;
        assert_eq!(head_len, hl2);
        assert_eq!(cycle_len, cl2);
        CharSeq {
            head: h1 & h2,
            cycle: c1 & c2,
            head_len,
            cycle_len,
        }
    }
}

impl CharSeq {
    /// Whether the formula accepts the trace,
    /// i.e. it is true starting from the first position.
    #[inline]
    pub(crate) fn accepts(&self) -> bool {
        (self.head & 1) == 1
    }

    /// LTL Next operator (X)
    #[inline]
    pub(crate) fn next(mut self) -> Self {
        let out_bit = self.cycle & 1;
        if self.head_len > 0 {
            self.head >>= 1;
            self.head |= out_bit << (self.head_len - 1);
        }

        if self.cycle_len > 0 {
            self.cycle >>= 1;
            self.cycle |= out_bit << (self.cycle_len - 1);
        }

        self
    }

    /// LTL Globally operator (G)
    #[inline]
    pub(crate) fn globally(self) -> Self {
        self.not().finally().not()
    }

    /// LTL Finally operator (F)
    #[inline]
    pub(crate) fn finally(self) -> Self {
        let CharSeq {
            head,
            cycle,
            head_len,
            cycle_len,
        } = self;
        if cycle > 0 {
            CharSeq {
                head: all_true_u64(head_len),
                cycle: all_true_u64(cycle_len),
                head_len,
                cycle_len,
            }
        } else {
            let mut x = head;
            x |= x >> 1;
            x |= x >> 2;
            x |= x >> 4;
            x |= x >> 8;
            x |= x >> 16;
            x |= x >> 32;
            CharSeq {
                head: x,
                cycle: 0,
                head_len,
                cycle_len,
            }
        }
    }

    /// LTL Until operator (U)
    #[inline]
    pub(crate) fn until(self, rhs: Self) -> Self {
        let CharSeq {
            head: h1,
            cycle: c1,
            head_len: hl1,
            cycle_len: cl1,
        } = self;
        let CharSeq {
            head: h2,
            cycle: c2,
            head_len: hl2,
            cycle_len: cl2,
        } = rhs;
        assert_eq!(hl1, hl2);
        assert_eq!(cl1, cl2);
        // Technique: double the cycles, and compute the regular Until
        // on them. Then, transmit a single bit to the head, and compute
        // until from there.
        let mut long_c1 = c1 as u128 | (c1 as u128) << cl1;
        let mut long_c2 = c2 as u128 | (c2 as u128) << cl2;
        long_c2 |= long_c1 & (long_c2 >> 1);
        long_c1 &= long_c1 >> 1;
        long_c2 |= long_c1 & (long_c2 >> 2);
        long_c1 &= long_c1 >> 2;
        long_c2 |= long_c1 & (long_c2 >> 4);
        long_c1 &= long_c1 >> 4;
        long_c2 |= long_c1 & (long_c2 >> 8);
        long_c1 &= long_c1 >> 8;
        long_c2 |= long_c1 & (long_c2 >> 16);
        long_c1 &= long_c1 >> 16;
        long_c2 |= long_c1 & (long_c2 >> 32);
        long_c1 &= long_c1 >> 32;
        long_c2 |= long_c1 & (long_c2 >> 64);
        println!("c2 {long_c2}");
        let cycle_res = restrict_to_first_k_bits(long_c2 as u64, cl1);
        let exit_bit = cycle_res & 1;

        let mut x = h1 as u128;
        let mut y = h2 as u128 | (exit_bit as u128) << hl2;
        y |= x & (y >> 1);
        x &= x >> 1;
        y |= x & (y >> 2);
        x &= x >> 2;
        y |= x & (y >> 4);
        x &= x >> 4;
        y |= x & (y >> 8);
        x &= x >> 8;
        y |= x & (y >> 16);
        x &= x >> 16;
        y |= x & (y >> 32);
        x &= x >> 32;
        y |= x & (y >> 64);
        let head_res = restrict_to_first_k_bits(y as u64, hl1);

        CharSeq {
            head: head_res,
            cycle: cycle_res,
            head_len: hl1,
            cycle_len: cl1,
        }
    }
}

impl Display for CharSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(head: ")?;
        for i in 0..self.head_len {
            write!(f, "{}", (self.head >> i) & 1)?;
        }
        write!(f, ", cycle: ")?;
        for i in 0..self.cycle_len {
            write!(f, "{}", (self.cycle >> i) & 1)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

fn u64_and_len_from_iter(iter: impl Iterator<Item = bool>) -> (u64, usize) {
    let mut x: u64 = 0;
    let mut len = 0;
    iter.enumerate().for_each(|(i, b)| {
        if i >= 64 {
            panic!("Trace is too long! (max len 64)");
        }
        if b {
            x |= 1 << i;
        }
        len += 1;
    });
    (x, len)
}

impl<It1, It2> From<(It1, It2)> for CharSeq
where
    It1: Iterator<Item = bool>,
    It2: Iterator<Item = bool>,
{
    fn from((it1, it2): (It1, It2)) -> Self {
        let (head, head_len) = u64_and_len_from_iter(it1);
        let (cycle, cycle_len) = u64_and_len_from_iter(it2);
        CharSeq {
            head,
            cycle,
            head_len,
            cycle_len,
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    #[allow(non_snake_case)]
    pub(crate) fn X(phi: CharSeq) -> CharSeq {
        phi.next()
    }

    #[allow(non_snake_case)]
    pub(crate) fn G(phi: CharSeq) -> CharSeq {
        phi.globally()
    }

    #[allow(non_snake_case)]
    pub(crate) fn F(phi: CharSeq) -> CharSeq {
        phi.finally()
    }

    #[allow(non_snake_case)]
    pub(crate) fn U(phi: CharSeq, psi: CharSeq) -> CharSeq {
        phi.until(psi)
    }

    fn random_seq_with_len(head_len: usize, cycle_len: usize, rng: &mut impl Rng) -> CharSeq {
        let h: u64 = restrict_to_first_k_bits(rng.gen(), head_len);
        let c: u64 = restrict_to_first_k_bits(rng.gen(), cycle_len);
        CharSeq {
            head: h,
            cycle: c,
            head_len,
            cycle_len,
        }
    }

    fn random_pair() -> (CharSeq, CharSeq) {
        let mut rng = thread_rng();
        let head_len = rng.gen_range(0..64);
        let cycle_len = rng.gen_range(0..64);
        (
            random_seq_with_len(head_len, cycle_len, &mut rng),
            random_seq_with_len(head_len, cycle_len, &mut rng),
        )
    }

    fn random_seq() -> CharSeq {
        let mut rng = thread_rng();
        let head_len = rng.gen_range(0..64);
        let cycle_len = rng.gen_range(0..64);
        random_seq_with_len(head_len, cycle_len, &mut rng)
    }

    #[test]
    fn phi_and_not_phi_is_zero() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!((x & !x).head, 0);
            assert_eq!((x & !x).cycle, 0);
        }
    }

    #[test]
    fn not_is_involutive() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(x, !!x);
        }
    }

    #[test]
    fn and_is_idempotent() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(x & x, x);
        }
    }

    #[test]
    fn or_is_idempotent() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(x | x, x);
        }
    }

    #[test]
    /// Test "DeMorgan" identities for LTL operators
    fn de_morgan_or_and() {
        for _ in 0..100 {
            let (x1, x2) = random_pair();
            assert_eq!(!(x1 | x2), !x1 & !x2);
        }
    }

    #[test]
    /// Test "DeMorgan" identities for LTL operators
    fn de_morgan_f_g() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(!F(x), G(!x));
        }
    }

    #[test]
    fn ff_is_f() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(F(F(x)), F(x));
        }
    }

    #[test]
    fn f_as_phi_or_x_f_phi() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(F(x), x | X(F(x)));
        }
    }

    #[test]
    fn and_distributes_g() {
        for _ in 0..100 {
            let (x, y) = random_pair();
            assert_eq!(G(x & y), G(x) & G(y));
        }
    }

    #[test]
    fn or_distributes_f() {
        for _ in 0..100 {
            let (x, y) = random_pair();
            assert_eq!(F(x | y), F(x) | F(y));
        }
    }

    #[test]
    fn gg_is_g() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(G(G(x)), G(x));
        }
    }

    #[test]
    fn expand_u() {
        for _ in 0..100 {
            let (x, y) = random_pair();
            println!("x = {x}, y = {y}");
            println!("{}, {}", U(x, y), y | (x & X(U(x, y))));
            assert_eq!(U(x, y), y | (x & X(U(x, y))));
        }
    }
}
