extern crate syntax;

use syntax::ext::base::{MacResult};
use syntax::util::small_vector::SmallVector;
use syntax::ast;
use std::gc::Gc;

pub struct MacroResult {
    items: Vec<Gc<ast::Item>>
}

impl MacroResult {
    pub fn new(items: Vec<Gc<ast::Item>>) -> Box<MacResult> {
        box MacroResult { items: items  } as Box<MacResult>
    }
}

impl MacResult for MacroResult {
    fn make_items(&self) -> Option<SmallVector<Gc<ast::Item>>> {
        Some(SmallVector::many(self.items.clone()))
    }
}
