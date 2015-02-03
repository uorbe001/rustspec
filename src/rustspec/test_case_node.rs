extern crate syntax;
extern crate rustc;
extern crate core;

use test_node::TestNode;

use self::core::ops::Deref;
use syntax::ext::base::ExtCtxt;
use syntax::codemap::DUMMY_SP;
use syntax::codemap::Spanned;
use syntax::ast_util::empty_generics;
use syntax::abi;
use syntax::ast;
use syntax::attr;
use syntax::ptr::P;
use syntax::parse::token;
use syntax::ext::build::AstBuilder;
use syntax::parse::token::InternedString;

fn is_item(stmt: &P<ast::Stmt>) -> bool {
    match stmt.node {
        ast::StmtDecl(ref decl, _) => {
            match decl.node {
                ast::DeclItem(_) => true,
                _ => false,
            }
        },
        _ => false,
    }
}

pub struct TestCaseNode {
    name: String,
    block: P<ast::Block>,
    should_fail: bool,
    should_be_ignored: bool
}

impl TestCaseNode {
    pub fn new(name: String,
               block: P<ast::Block>,
               should_fail: bool,
               should_be_ignored: bool
              ) -> Box<TestCaseNode> {
        Box::new(TestCaseNode { name: name, block: block, should_fail: should_fail, should_be_ignored: should_be_ignored })
    }

    fn build_test_attributes(&self, cx: &mut ExtCtxt) -> Vec<Spanned<ast::Attribute_>> {
        let mut attributes = vec![];

        attributes.push(cx.attribute(DUMMY_SP, cx.meta_word(DUMMY_SP, token::InternedString::new("test"))));
        attributes.push(attr::mk_attr_outer(attr::mk_attr_id(), attr::mk_list_item(
                InternedString::new("allow"),
                vec!(attr::mk_word_item(InternedString::new("non_snake_case")))
            )
        ));

        if self.should_fail {
            attributes.push(cx.attribute(DUMMY_SP, cx.meta_word(DUMMY_SP, token::InternedString::new("should_fail"))));
        }

        if self.should_be_ignored {
            attributes.push(cx.attribute(DUMMY_SP, cx.meta_word(DUMMY_SP, token::InternedString::new("ignore"))));
        }

        attributes
    }
}

impl TestNode for TestCaseNode {
    fn to_item(&self, cx: &mut ExtCtxt, before_blocks: &mut Vec<P<ast::Block>>) -> P<ast::Item> {
        let body = if before_blocks.is_empty() {
            self.block.clone()
        } else {
            let block = self.block.deref().clone();

            let mut before_view_items: Vec<P<ast::Stmt>> = vec![];
            let mut before_stmts: Vec<P<ast::Stmt>> = vec![];

            for before_block in before_blocks.iter() {
                let items: Vec<P<ast::Stmt>> = before_block.stmts.clone().into_iter().filter(is_item).collect();
                before_view_items.push_all(items.as_slice());
                before_stmts.push_all(before_block.stmts.as_slice());
            }

            P(ast::Block {
                stmts: before_view_items + before_stmts.as_slice() + block.stmts.as_slice(),
                ..block
            })
        };

        P(ast::Item {
            ident: cx.ident_of(self.get_name().as_slice()),
            attrs: self.build_test_attributes(cx),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemFn(
                cx.fn_decl(
                    Vec::new(),
                    P(ast::Ty {
                        id: ast::DUMMY_NODE_ID,
                        node: ast::TyTup(vec![]),
                        span: DUMMY_SP,
                    })
                ),
                ast::Unsafety::Normal,
                abi::Rust,
                empty_generics(),
                body.clone()
            ),
            vis: ast::Inherited,
            span: body.span,
        })
    }

    fn get_name(&self) -> String {
        self.name.replace(" ", "_").replace("$", "")
    }
}

