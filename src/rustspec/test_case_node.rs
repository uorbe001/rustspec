extern crate syntax;
extern crate rustc;

use test_node::TestNode;

use std::gc::{Gc, GC};
use syntax::ext::base::ExtCtxt;
use syntax::codemap::DUMMY_SP;
use syntax::ast_util::empty_generics;
use syntax::abi;
use syntax::ast;
use syntax::parse::token;
use syntax::ext::build::AstBuilder;

pub struct TestCaseNode {
    name: String,
    block: Gc<syntax::ast::Block>
}

impl TestCaseNode {
    pub fn new(name: String,
               block: Gc<syntax::ast::Block>
              ) -> Box<TestCaseNode> {
        box TestCaseNode { name: name, block: block }
    }
}

impl TestNode for TestCaseNode {
    fn to_item(&self, cx: &mut ExtCtxt, before_blocks: &mut Vec<Gc<syntax::ast::Block>>) -> Gc<ast::Item> {
        let body = if before_blocks.is_empty() {
            self.block
        } else {
            let block = self.block.deref().clone();

            let (before_view_items, before_stmts) = before_blocks.iter().fold(
                (vec![], vec![]),
                |(view_accum, stmts_accum), b|
                (view_accum + b.view_items, stmts_accum + b.stmts)
            );

            box(GC) ast::Block {
                view_items: before_view_items + block.view_items,
                stmts: before_stmts + block.stmts,
                ..block
            }
        };

        let attr_test = cx.attribute(DUMMY_SP,
             cx.meta_word(DUMMY_SP, token::InternedString::new("test")));

        box(GC) ast::Item {
            ident: cx.ident_of(self.get_name().as_slice()),
            attrs: vec!(attr_test),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemFn(
                cx.fn_decl(Vec::new(), cx.ty_nil()),
                ast::NormalFn,
                abi::Rust,
                empty_generics(),
                body
            ),
            vis: ast::Inherited,
            span: body.span,
        }
    }

    fn get_name(&self) -> String {
        self.name.replace(" ", "_").replace("$", "")
    }
}

