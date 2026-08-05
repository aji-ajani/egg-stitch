use egg::Symbol;
use std::fmt::{self, Display, Formatter};
use super::{StitchDisc, StitchOp, Weights};

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum TsOp {
    Define,
    Done, // change to pass
    Lam,
    Var(i32),
    Sym(Symbol)
}

impl Display for TsOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Define => f.write_str("define"),
            Self::Var(n) => write!(f, "${n}"),
            Self::Sym(s) => Display::fmt(s, f),
        }
    }
}

impl StitchDisc for TsOp {
    fn de_bruijn_index(&self) -> Option<i32> {
        match self { Self::Var(n) => Some(*n), _ => None }
    }

    fn binds_child(&self, j: usize) -> bool {
        match self {
            Self::Define => j == 1,
            Self::Lam    => j == 0,
            _ => false,
        }
    }
    // intrinsic_size / as_var: maintain defaults
}

impl StitchOp for TsOp {
    fn from_name(s: &str) -> Self {
        if let Some(rest) = s.strip_prefix('$')
            && let Ok(n) = rest.parse::<i32>() { return Self::Var(n); }
        match s {
            "define" => Self::Define,
            "seq" => Self::Seq,
        }
    }
}