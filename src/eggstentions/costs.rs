use egg::{CostFunction, SymbolLang, Id};
use std::collections::HashSet;
use std::cmp::Ordering;
use itertools::Itertools;
use symbolic_expressions::encode_string;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepOrder {
    vars: HashSet<String>,
    depth: usize,
    size: usize,
}

impl PartialOrd for RepOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.size.partial_cmp(&other.size) {
            None => { self.vars.len().partial_cmp(&other.vars.len()) }
            Some(x) => {
                match x {
                    Ordering::Less => {Some(Ordering::Less)}
                    Ordering::Equal => {other.vars.len().partial_cmp(&self.vars.len()) }
                    Ordering::Greater => {Some(Ordering::Greater)}
                }
            }
        }
    }
}

pub struct MinRep;

impl CostFunction<SymbolLang> for MinRep {
    /// The `Cost` type. It only requires `PartialOrd` so you can use
    /// floating point types, but failed comparisons (`NaN`s) will
    /// result in a panic.
    /// We choose to count
    type Cost = RepOrder;

    /// Calculates the cost of an enode whose children are `Cost`s.
    ///
    /// For this to work properly, your cost function should be
    /// _monotonic_, i.e. `cost` should return a `Cost` greater than
    /// any of the child costs of the given enode.
    fn cost<C>(&mut self, enode: &SymbolLang, mut costs: C) -> Self::Cost where
        C: FnMut(Id) -> Self::Cost {
        let current_depth = enode.children.iter().map(|i| costs(*i).depth).max().unwrap_or(0);
        let current_size = enode.children.iter().map(|i| costs(*i).size).sum1().unwrap_or(0);
        let mut vars = enode.children.iter().flat_map(|i| costs(*i).vars).collect::<HashSet<String>>();
        if enode.op.as_str().starts_with("ts_ph") {
            vars.insert(enode.op.to_string());
        }
        RepOrder{depth: current_depth + 1, size: current_size + 1, vars}
    }
}

#[cfg(test)]
mod tests {
    use crate::eggstentions::costs::RepOrder;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn compare_two_different_sizes() {
        assert!(RepOrder{vars: HashSet::new(), depth: 0, size: 1} < RepOrder{vars: HashSet::new(), depth: 0, size: 2});
        assert!(RepOrder{vars: HashSet::from_iter(vec![":".to_string(), "a".to_string(), "b".to_string()]), depth: 0, size: 1} < RepOrder{vars: HashSet::new(), depth: 0, size: 2});
    }

    #[test]
    fn compare_two_different_vars() {
        assert!(RepOrder{vars: HashSet::from_iter(vec![":".to_string(), "a".to_string(), "b".to_string()]), depth: 0, size: 2} < RepOrder{vars:  HashSet::from_iter(vec![":".to_string(), "a".to_string()]), depth: 0, size: 2});
    }
}