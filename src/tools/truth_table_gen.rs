use std::{collections::{HashMap, HashSet}};
use ascii_table::AsciiTable;

use crate::prop::PropAstNode;
use wasm_bindgen::prelude::*;

fn collect_symbols(prop: &PropAstNode) -> HashSet<String> {
    match prop {
        PropAstNode::Symbol(sym) => {
            let mut set = HashSet::new();
            set.insert(sym.clone());
            set
        },
        PropAstNode::Value(_) => HashSet::new(),
        PropAstNode::Not(x) => collect_symbols(x),
        PropAstNode::BinOp(_, l, r) => {
            let mut set = collect_symbols(l);
            set.extend(collect_symbols(r));
            set
        },
    }
}

fn collect_components(prop: &PropAstNode) -> Vec<&PropAstNode> {
    match prop {
        PropAstNode::Symbol(_) => Vec::new(),
        PropAstNode::Value(_) => Vec::new(),
        PropAstNode::Not(x) => {
            let mut res = collect_components(x);
            res.push(prop);
            res
        },
        PropAstNode::BinOp(_, l, r) => {
            let mut res = collect_components(l);
            for comp in collect_components(r) {
                if !res.contains(&comp) {
                    res.push(comp);
                }
            }
            res.push(prop);
            res
        },
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Valuation {
    pub symbol: String,
    pub value: bool,
}

#[wasm_bindgen(getter_with_clone)]
pub struct TruthTableEntry {
    pub bv: Vec<Valuation>,
    pub comp: Vec<Valuation>,
}

pub fn print_truth_table(entries: Vec<TruthTableEntry>) {
    let mut ascii_table = AsciiTable::default();
    let mut idx = 0;

    for val in &entries[0].bv{
        ascii_table.column(idx).set_header(val.symbol.clone()).set_align(ascii_table::Align::Center);
        idx += 1;
    }
    for val in &entries[0].comp{
        ascii_table.column(idx).set_header(val.symbol.clone()).set_align(ascii_table::Align::Center);
        idx += 1;
    }

    let mut data = vec![];
    
    for entry in &entries{
        let mut row = vec![];
        for ele in &entry.bv{
            row.push(if ele.value {"T"} else {"F"})
        }
        for ele in &entry.comp{
            row.push(if ele.value {"T"} else {"F"})
        }
        data.push(row)
    }

    ascii_table.print(data)
}

pub fn truth_table_gen(prop: &PropAstNode) -> Vec<TruthTableEntry> {
    let mut symbols: Vec<String> = collect_symbols(prop).into_iter().collect();
    symbols.sort_by(|a, b| a.cmp(b));
    
    let comps = collect_components(prop);
    let mut results = Vec::new();
    for bv_bin in 0..(1u64<<symbols.len()) {
        let mut bv: Vec<Valuation> = Vec::new();
        let mut bv_map: HashMap<String, bool> = HashMap::new();
        for (i, sym) in symbols.iter().enumerate() {
            bv.push(Valuation {
                symbol: sym.clone(), 
                value: (bv_bin & (1u64<<(symbols.len()-1-i))) != 0,
            });
            bv_map.insert(sym.clone(), (bv_bin & (1u64<<(symbols.len()-1-i))) != 0);
        }
        results.push(TruthTableEntry {
            bv: bv.clone(),
            comp: comps.iter().map(|x| Valuation {
                symbol: x.to_string(), 
                value: x.eval(&bv_map).unwrap()
            }).collect(),
        });
    }
    results
}
