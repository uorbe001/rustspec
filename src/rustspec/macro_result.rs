extern crate syntax;

use syntax::ext::base::{MacResult};
use syntax::util::small_vector::SmallVector;
use syntax::ast;
use syntax::ptr::P;

pub struct MacroResult {
    items: Vec<P<ast::Item>>
}

impl MacroResult {
    pub fn new(items: Vec<P<ast::Item>>) -> Box<MacResult + 'static> {
        box MacroResult { items: items  } as Box<MacResult>
    }
}

impl MacResult for MacroResult {
    fn make_items(self: Box<MacroResult>) -> Option<SmallVector<P<ast::Item>>> {
        Some(SmallVector::many(self.items.clone()))
    }
}
