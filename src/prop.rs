use std::collections::HashMap;
use std::hash::Hash;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use anyhow::{Result, format_err};

#[derive(Parser)]
#[grammar = "src/prop.pest"]
pub struct PropParser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropBinOp {
    And,
    Or,
    Implies,
    Iff,
}

fn eval_op(op: &PropBinOp, l: Result<bool>, r: Result<bool>) -> Result<bool> {
    match op {
        PropBinOp::And => Ok(l? && r?),
        PropBinOp::Or => Ok(l? || r?),
        PropBinOp::Implies => Ok(!(l?) || r?),
        PropBinOp::Iff => Ok(l? == r?),
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PropAstNode {
    Symbol(String),
    Value(bool),
    Not(Box<PropAstNode>),
    BinOp(PropBinOp, Box<PropAstNode>, Box<PropAstNode>),
}

impl PropAstNode {
    pub fn eval(&self, boolean_value_map: &HashMap<String, bool>) -> Result<bool> {
        match self {
            PropAstNode::Symbol(sym) => boolean_value_map.get(sym).ok_or(format_err!("symbol not found: {}", sym)).map(|x| *x),
            PropAstNode::Value(x) => Ok(*x),
            PropAstNode::Not(x) => Ok(!(x.eval(boolean_value_map)?)),
            PropAstNode::BinOp(op, l, r) => eval_op(op, l.eval(boolean_value_map), r.eval(boolean_value_map)),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            PropAstNode::Symbol(sym) => sym.clone(),
            PropAstNode::Value(x) => x.to_string(),
            PropAstNode::Not(x) => format!("!{}", x.to_string()),
            PropAstNode::BinOp(op, l, r) => format!("({} {} {})", l.to_string(), match op {
                PropBinOp::And => "&",
                PropBinOp::Or => "|",
                PropBinOp::Implies => "=>",
                PropBinOp::Iff => "<=>",
            }, r.to_string()),
        }
    }
}

fn build_ast_bin_op_node_from_pairs(binop: PropBinOp, mut pairs: Vec<Pair<'_, Rule>>) -> Result<Box<PropAstNode>> {
    if pairs.len() == 0 {
        return Err(format_err!("expected at least one expr"));
    }
    let first = pairs.remove(0);
    if pairs.len() == 0 {
        build_ast_node_from_pair(first)
    } else {
        pairs.remove(0);
        Ok(Box::new(PropAstNode::BinOp(binop, build_ast_node_from_pair(first)?, build_ast_bin_op_node_from_pairs(binop, pairs)?)))
    }
}

fn build_ast_node_from_pair(pair: Pair<'_, Rule>) -> Result<Box<PropAstNode>> {
    match pair.as_rule() {
        Rule::expr | Rule::atom => {
            let mut inner_pairs = pair.into_inner();
            let first = inner_pairs.next().ok_or(format_err!("expected at least one expr"))?;
            build_ast_node_from_pair(first)
        },
        Rule::and_expr | Rule::or_expr | Rule::implies_expr | Rule::iff_expr => {
            let binop = match pair.as_rule() {
                Rule::and_expr => PropBinOp::And,
                Rule::or_expr => PropBinOp::Or,
                Rule::implies_expr => PropBinOp::Implies,
                Rule::iff_expr => PropBinOp::Iff,
                _ => unreachable!(),
            };
            let inner_pairs: Vec<Pair<'_, Rule>> = pair.into_inner().collect();
            build_ast_bin_op_node_from_pairs(binop, inner_pairs)
        },
        Rule::not_expr => {
            let mut inner_pairs: Vec<Pair<'_, Rule>> = pair.into_inner().collect();
            if inner_pairs.len() == 1 {
                return build_ast_node_from_pair(inner_pairs.remove(0));
            }
            Ok(Box::new(PropAstNode::Not(build_ast_node_from_pair(inner_pairs.remove(1))?)))
        },
        Rule::symbol => {
            Ok(Box::new(PropAstNode::Symbol(pair.as_str().to_string())))
        },
        Rule::r#false | Rule::r#true => {
            Ok(Box::new(PropAstNode::Value(pair.as_str() == "true")))
        },
        _ => {
            Err(format_err!("unexpected rule: {:?}", pair.as_rule()))
        }
    }
}

pub fn parse_prop(input: &str) -> Result<Box<PropAstNode>> {
    let parsed = PropParser::parse(Rule::expr, input)?;
    for pair in parsed {
        return build_ast_node_from_pair(pair)
    }
    return Err(anyhow::anyhow!("expected exactly one expr"));
}
