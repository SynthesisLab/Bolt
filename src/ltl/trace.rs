use std::{collections::HashMap, fs::File, io::Read, path::Path};

use serde::Deserialize;

use crate::ops::{binary::LtlBinaryOp, unary::LtlUnaryOp};

use super::cs::CharSeq;

#[derive(Debug, Clone)]
pub struct Instance {
    pub traces: Vec<Trace>,
    pub atomic_propositions: Vec<String>,
    pub target: Vec<bool>,
    pub operators: Operators,
}

/// Stores the [`CharSeq`] of each predicate on a given trace.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Trace {
    pub alphabet: Vec<CharSeq>,
}

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

    fn all() -> Self {
        Self {
            unary: LtlUnaryOp::all(),
            binary: LtlBinaryOp::all(),
        }
    }

    pub fn len(&self) -> usize {
        self.unary.len() + self.binary.len()
    }
}

pub fn traces_from_file(fname: impl AsRef<Path>) -> Instance {
    let mut file = File::open(fname).expect("Failed to open trace file");

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to read trace file.");

    parse_traces(&buf)
}

pub fn parse_traces(buf: &str) -> Instance {
    let parsed_input: ParsedInput = serde_json::from_str(buf).expect("Failed to parse input json");
    assert!(
        parsed_input.max_length_traces <= 64,
        "Input traces are too long: {} (max 64)",
        parsed_input.max_length_traces
    );

    let traces: Option<Vec<_>> = parsed_input
        .positive_traces
        .into_iter()
        .map(|pt| pt.traces_vec_from_alphabet(&parsed_input.atomic_propositions))
        .chain(
            parsed_input
                .negative_traces
                .into_iter()
                .map(|pt| pt.traces_vec_from_alphabet(&parsed_input.atomic_propositions)),
        )
        .collect();
    let traces = traces.expect("Incorrect traces format");

    let target: Vec<_> = (0..parsed_input.number_positive_traces)
        .map(|_| true)
        .chain((0..parsed_input.number_negative_traces).map(|_| false))
        .collect();

    // Operators filtering is not implemented in current input format.
    let operators = Operators::all();

    Instance {
        traces,
        atomic_propositions: parsed_input.atomic_propositions,
        target,
        operators,
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParsedTrace {
    #[serde(flatten)]
    alphabet: HashMap<String, Vec<String>>,
}

/// Converts a vector of "0"/"1" strings to a CharSeq
fn vec_to_cs(v: &Vec<String>) -> CharSeq {
    v.iter()
        .map(|c| match c {
            s if s == "0" => false,
            s if s == "1" => true,
            _ => panic!("Unexpected literal in traces definition"),
        })
        .collect()
}

impl ParsedTrace {
    pub fn traces_vec_from_alphabet(&self, alphabet: &Vec<String>) -> Option<Trace> {
        let char_seqs: Option<Vec<CharSeq>> = alphabet
            .iter()
            .map(|s| self.alphabet.get(s).map(vec_to_cs))
            .collect();
        Some(Trace {
            alphabet: char_seqs?,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParsedInput {
    positive_traces: Vec<ParsedTrace>,
    negative_traces: Vec<ParsedTrace>,
    atomic_propositions: Vec<String>,
    number_positive_traces: usize,
    number_negative_traces: usize,
    max_length_traces: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_vec_to_cs() {
        let v = vec!["1".into(), "0".into(), "1".into()];
        let cs = vec_to_cs(&v);
        assert_eq!(cs.len(), 3);
        assert_eq!(cs.values, 0b101);
    }

    #[test]
    fn parsing() {
        let buf = r#"
{
    "positive_traces": [
        {
            "a0": ["0", "1", "1", "0"],
            "a1": ["0", "1", "0", "1"]
        }
    ],
    "negative_traces": [
        {
            "a0": ["1", "1", "1", "0"],
            "a1": ["0", "1", "0", "1"]
        },
        {
            "a0": ["0", "1", "1", "0"],
            "a1": ["0", "1", "1", "1"]
        },
        {
            "a0": ["0", "1", "1", "1"],
            "a1": ["0", "1", "0", "1"]
        },
        {
            "a0": ["0", "1", "1", "0"],
            "a1": ["0", "1", "0", "0"]
        }
    ],
    "smallest_known_formula": "",
    "generating_formula": "",
    "generating_seed": "",
    "original_repository": "https://github.com/MojtabaValizadeh/ltl-learning-on-gpus",
    "name": "",
    "atomic_propositions": ["a0","a1"],
    "number_atomic_propositions": 2,
    "number_traces": 5,
    "number_positive_traces": 1,
    "number_negative_traces": 4,
    "max_length_traces": 4,
    "trace_type": "finite"
}"#;
        let _res = parse_traces(buf);
    }
}
