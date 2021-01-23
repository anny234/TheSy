use egg::{Analysis, Applier, EGraph, Id, Language, Pattern, SearchMatches, Subst, SymbolLang, Var};
use itertools::Itertools;

pub struct CompApplier {
    appliers: Vec<Pattern<SymbolLang>>
}

impl CompApplier {
    pub fn new(appliers: Vec<Pattern<SymbolLang>>) -> CompApplier {
        CompApplier{appliers}
    }
}

impl Applier<SymbolLang, ()> for CompApplier {
    fn apply_matches(&self, egraph: &mut EGraph<SymbolLang, ()>, matches: &[SearchMatches]) -> Vec<Id> {
        let mut added = vec![];
        for mat in matches {
            for subst in &mat.substs {
                let ids = self.appliers.iter().flat_map(|a| a.apply_one(egraph, mat.eclass, subst)).collect_vec();
                let eclass = ids[0];
                let added_ids = ids.into_iter().filter_map(|i| {
                    let (to, did_something) = egraph.union(i, eclass);
                    if did_something {
                        Some(to)
                    } else {
                        None
                    }
                });
                added.extend(added_ids)
            }
        };
        added
    }

    fn apply_one(&self, egraph: &mut EGraph<SymbolLang, ()>, eclass: Id, subst: &Subst) -> Vec<Id> {
        unimplemented!()
    }
}

pub struct DiffApplier<T: Applier<SymbolLang, ()>> {
    applier: T
}

impl<T: Applier<SymbolLang, ()>> DiffApplier<T> {
    pub fn new(applier: T) -> DiffApplier<T> {
        DiffApplier { applier }
    }
}

impl DiffApplier<Pattern<SymbolLang>> {
    pub fn pretty(&self, width: usize) -> String {
        self.applier.pretty(width)
    }
}

impl<T: Applier<SymbolLang, ()>> Applier<SymbolLang, ()> for DiffApplier<T> {
    fn apply_matches(&self, egraph: &mut EGraph<SymbolLang, ()>, matches: &[SearchMatches]) -> Vec<Id> {
        let mut added = vec![];
        for mat in matches {
            for subst in &mat.substs {
                let ids = self.apply_one(egraph, mat.eclass, subst);
                //     .into_iter()
                //     .filter_map(|id| {
                //         let (to, did_something) = egraph.union(id, mat.eclass);
                //         if did_something {
                //             Some(to)
                //         } else {
                //             None
                //         }
                //     });
                // added.extend(ids)
            }
        }
        added
    }

    fn apply_one(&self, egraph: &mut EGraph<SymbolLang, ()>, eclass: Id, subst: &Subst) -> Vec<Id> {
        self.applier.apply_one(egraph, eclass, subst)
    }
}

pub struct UnionApplier {
    vars: Vec<Var>,
}

impl UnionApplier {
    pub fn new(vars: Vec<Var>) -> UnionApplier {
        UnionApplier{vars}
    }
}

impl<L: Language, N: Analysis<L>> Applier<L, N> for UnionApplier {
    fn apply_matches(&self, egraph: &mut EGraph<L, N>, matches: &[SearchMatches]) -> Vec<Id> {
        let mut added = vec![];
        for mat in matches {
            for subst in &mat.substs {
                let first = self.vars.first().unwrap();
                let ids = self.vars.iter().skip(1).filter_map(|v| {
                    let (to, did_something) = egraph.union(*subst.get(*first).unwrap(), *subst.get(*v).unwrap());
                    if did_something {
                        Some(to)
                    } else {
                        None
                    }
                    }).collect_vec();
                added.extend(ids)
            }
        }
        added
    }

    fn apply_one(&self, egraph: &mut EGraph<L, N>, eclass: Id, subst: &Subst) -> Vec<Id> {
        unimplemented!()
    }


    fn vars(&self) -> Vec<Var> {
        self.vars.clone()
    }
}