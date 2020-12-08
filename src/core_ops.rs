use super::types::{EvalErr, Reduction};
use crate::node::{Allocator, Node, U8};

const FIRST_COST: u32 = 10;
const IF_COST: u32 = 10;
const CONS_COST: u32 = 10;
const REST_COST: u32 = 10;
const LISTP_COST: u32 = 10;

impl Node {
    pub fn first(&self) -> Result<Node, EvalErr<Node>> {
        match self.pair() {
            Some((a, _b)) => Ok(a),
            _ => self.err("first of non-cons"),
        }
    }

    pub fn rest(&self) -> Result<Node, EvalErr<Node>> {
        match self.pair() {
            Some((_a, b)) => Ok(b),
            _ => self.err("rest of non-cons"),
        }
    }
}

pub fn op_if<T>(
    _allocator: &dyn Allocator<Node, T>,
    args: &Node,
) -> Result<Reduction<Node>, EvalErr<Node>> {
    let cond = args.first()?;
    let mut chosen_node = args.rest()?;
    if cond.nullp() {
        chosen_node = chosen_node.rest()?;
    }
    Ok(Reduction(IF_COST, chosen_node.first()?))
}

pub fn op_cons<T>(
    allocator: &dyn Allocator<Node, T>,
    args: &Node,
) -> Result<Reduction<Node>, EvalErr<Node>> {
    let a1 = args.first()?;
    let a2 = args.rest()?.first()?;
    Ok(Reduction(CONS_COST, allocator.from_pair(&a1, &a2)))
}

pub fn op_first<T>(
    _allocator: &dyn Allocator<Node, T>,
    args: &Node,
) -> Result<Reduction<Node>, EvalErr<Node>> {
    Ok(Reduction(FIRST_COST, args.first()?.first()?))
}

pub fn op_rest<T>(
    _allocator: &dyn Allocator<Node, T>,
    args: &Node,
) -> Result<Reduction<Node>, EvalErr<Node>> {
    Ok(Reduction(REST_COST, args.first()?.rest()?))
}

pub fn op_listp(
    allocator: &dyn Allocator<Node, U8>,
    args: &Node,
) -> Result<Reduction<Node>, EvalErr<Node>> {
    match args.first()?.pair() {
        Some((_first, _rest)) => Ok(Reduction(LISTP_COST, allocator.one())),
        _ => Ok(Reduction(LISTP_COST, allocator.null())),
    }
}

pub fn op_raise<T>(
    _allocator: &dyn Allocator<Node, T>,
    args: &Node,
) -> Result<Reduction<Node>, EvalErr<Node>> {
    args.err("clvm raise")
}

pub fn op_eq(
    allocator: &dyn Allocator<Node, U8>,
    args: &Node,
) -> Result<Reduction<Node>, EvalErr<Node>> {
    let a0 = args.first()?;
    let a1 = args.rest()?.first()?;
    if let Some(s0) = a0.atom() {
        if let Some(s1) = a1.atom() {
            let cost: u32 = s0.len() as u32 + s1.len() as u32;
            return Ok(Reduction(
                cost,
                if s0 == s1 {
                    allocator.one()
                } else {
                    allocator.null()
                },
            ));
        }
    }
    args.err("= on list")
}
