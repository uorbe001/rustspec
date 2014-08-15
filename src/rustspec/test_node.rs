extern crate syntax;

use std::gc::Gc;
use syntax::ext::base::ExtCtxt;
use syntax::ast;

pub trait TestNode {
    fn to_item(&self, cx: &mut ExtCtxt, before_blocks: &mut Vec<Gc<syntax::ast::Block>>) -> Gc<ast::Item>;
    fn get_name(&self) -> String;
}
