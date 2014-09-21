extern crate syntax;

use syntax::ext::base::ExtCtxt;
use syntax::ast;
use syntax::ptr::P;

pub trait TestNode {
    fn to_item(&self, cx: &mut ExtCtxt, before_blocks: &mut Vec<P<ast::Block>>) -> P<ast::Item>;
    fn get_name(&self) -> String;
}
