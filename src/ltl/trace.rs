use std::{fs::File, io::Read, path::Path};

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

fn parse_trace(trace: &str) -> Option<Trace> {
    let seq_pred: Vec<_> = trace
        .split(';')
        .map(|s| s.split(',').map(|v| v == "1").collect::<Vec<_>>())
        .collect();

    let n_pred = seq_pred.first()?.len();
    let alphabet = (0..n_pred)
        .map(|i| CharSeq::from_iter(seq_pred.iter().map(|v| v[i])))
        .collect();

    Some(Trace { alphabet })
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
                .filter_map(parse_trace)
                .collect::<Vec<_>>()
        })
        .collect();

    let op_desc = buf
        .split("---")
        .nth(2)
        .expect("No operators list.")
        .trim_matches('\n');

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
        let buf = "0,0;0,1;0,0;0,1;0,0
0,1;0,0;0,1;0,1;0,0
0,1;0,0;0,1;0,0;0,1
0,1;0,1;0,0;0,0;0,1
0,1;0,0;0,0;0,1;0,1
0,0;0,1;0,0;0,0;0,1
0,0;0,1;0,1;0,0;0,0
0,1;0,0;0,0;0,1;0,1
0,1;0,1;0,1;0,1;0,1
0,1;0,0;0,1;0,0;0,1
---
1,0;0,1;0,0;0,1;0,1
1,0;0,1;1,1;1,0;1,0
0,0;1,1;0,0;0,1;1,1
0,1;0,1;1,1;0,1;1,0
1,0;1,0;1,0;1,0;1,0
0,1;1,1;1,1;0,1;0,1
1,0;1,1;0,1;0,1;0,0
1,0;1,0;1,1;1,0;0,1
1,1;0,1;0,0;0,1;1,0
0,0;0,0;1,1;1,0;0,1
---
F,G,X,!,&,|
---
p,q";
        let _res = parse_traces(buf);
    }
}
