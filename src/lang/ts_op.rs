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
            Self::Done => f.write_str("done"),
            Self::Lam => f.write_str("lam"),
            Self::Var(n) => write!(f, "${n}"),
            Self::Sym(s) => Display::fmt(s, f),
        }
    }
}

impl StitchDisc for TsOp {
    fn intrinsic_size(&self, weights: &Weights) -> u32 {
        weights.sym_var_cost
    }

    fn as_var(&self) -> Option<egg::Var> {
        None
    }

    fn de_bruijn_index(&self) -> Option<i32> {
        match self {
            Self::Var(n) => Some(*n),
            _ => None
        }
    }

    fn binds_child(&self, j: usize) -> u32 {
        match self {
            Self::Define => if j == 1 { 1 } else { 0 },
            Self::Lam => if j == 0 { 1 } else { 0 }, // ! update for lams with arity >1
            _ => 0,
        }
    }
}

impl StitchOp for TsOp {
    fn from_name(s: &str) -> Self {
        if let Some(rest) = s.strip_prefix('$')
            && let Ok(n) = rest.parse::<i32>() { return Self::Var(n); }
        match s {
            "define" => Self::Define,
            "done" => Self::Done,
            "lam" => Self::Lam,
            _ => Self::Sym(Symbol::from(s)),
        }
    }

    fn make_db_var(n: i32) -> Option<Self> {
        Some(Self::Var(n))
    }
}