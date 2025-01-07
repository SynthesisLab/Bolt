use std::{fs::File, io::Read, path::Path};

use log::debug;

use crate::ops::{binary::LtlBinaryOp, unary::LtlUnaryOp};

use super::cs::CharSeq;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Operators {
    pub(crate) unary: Vec<LtlUnaryOp>,
    pub(crate) binary: Vec<LtlBinaryOp>,
}

impl Operators {
    pub(crate) fn filter_bool(mut self) -> Self {
        self.unary.retain(|op| op.is_boolean());
        self.binary.retain(|op| op.is_boolean());
        self
    }

    pub fn len(&self) -> usize {
        self.unary.len() + self.binary.len()
    }
}

/// Stores the [`CharSeq`] of each predicate on a given trace.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Trace {
    pub alphabet: Vec<CharSeq>,
}

fn parse_trace(trace: &str) -> Trace {
    debug!("{trace}");
    let mut pieces = trace.split('|');
    let predicates_heads = pieces
        .next()
        .expect("Missing predicates heads.")
        .split(';')
        .filter(|s| !s.is_empty())
        .map(|s| s.split(',').map(|v| v == "1").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let predicates_cycle = pieces
        .next()
        .expect("Missing predicates cycles.")
        .split(';')
        .filter(|s| !s.is_empty())
        .map(|s| s.split(',').map(|v| v == "1").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n_pred = predicates_cycle.first().expect("No predicates").len();
    let alphabet = (0..n_pred)
        .map(|i| {
            (
                predicates_heads.iter().map(|v| v[i]),
                predicates_cycle.iter().map(|v| v[i]),
            )
                .into()
        })
        .collect();

    Trace { alphabet }
}

pub fn traces_from_file(
    fname: impl AsRef<Path>,
) -> (Vec<Trace>, Vec<String>, Vec<bool>, Operators) {
    let mut file = File::open(fname).expect("Failed to open trace file");

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to read trace file.");

    parse_traces(&buf)
}

pub(crate) fn parse_traces(buf: &str) -> (Vec<Trace>, Vec<String>, Vec<bool>, Operators) {
    let mut traces: Vec<_> = buf
        .split("---")
        .take(2)
        .map(|trs| {
            trs.trim_matches('\n')
                .lines()
                .map(|t| parse_trace(t))
                .collect::<Vec<_>>()
        })
        .collect();

    let op_desc = buf
        .split("---")
        .nth(2)
        .expect("No operators section.")
        .trim_matches('\n')
        .split('\n')
        .next()
        .expect("No operators list.");

    let operators = if op_desc == "All Operators" {
        Operators {
            unary: LtlUnaryOp::all(),
            binary: LtlBinaryOp::all(),
        }
    } else {
        let unary = op_desc
            .split(',')
            .filter_map(|s| LtlUnaryOp::try_from(s).ok())
            .collect::<Vec<_>>();
        let binary = op_desc
            .split(',')
            .filter_map(|s| LtlBinaryOp::try_from(s).ok())
            .collect::<Vec<_>>();
        debug!("Operators: {unary:?} {binary:?} {op_desc:?}");
        Operators { unary, binary }
    };

    let alphabet = buf
        .split("---")
        .skip(3)
        .take(1)
        .map(|trs| {
            trs.trim_matches('\n')
                .split(',')
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .next()
        .expect("No alphabet definition.");

    let target = traces[0]
        .iter()
        .map(|_| true)
        .chain(traces[1].iter().map(|_| false))
        .collect();

    let neg = traces.pop().expect("Negative instances");
    let mut traces = traces.pop().expect("Positive instances");

    traces.extend(neg);
    (traces, alphabet, target, operators)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        let buf = "|1,1;0,0;1,1;0,1
1,0;|1,0;1,1;0,1;1,1
0,0;1,0;|1,1;1,0;0,0
1,1;|1,0;0,1;1,1;0,1
0,0;1,1;|0,1
0,1;0,0;0,1;|0,1;1,1;1,0
0,1;1,0;0,1;|0,1;1,0;0,0
1,0;0,1;|1,1;0,1;1,0;1,1
---
1,1;|0,0;1,0;1,0;0,0
0,1;0,1;|0,1;1,0;0,0
|1,0;0,0;1,1;1,0
0,0;1,1;|1,0;0,0;0,1
1,0;0,0;1,0;|0,0;1,0;1,0
0,0;|0,1;0,0;1,0;0,0
0,0;1,1;|0,0;0,1;1,0;0,0
0,1;|0,0;1,0;1,0;0,0;1,0
---
All Operators
F(p & X(q))
---
p,q";
        let _res = parse_traces(buf);
    }
}
