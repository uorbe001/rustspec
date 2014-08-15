extern crate syntax;

use test_node::TestNode;

use std::gc::{Gc, GC};
use syntax::ext::base::ExtCtxt;
use syntax::codemap;
use syntax::codemap::DUMMY_SP;
use syntax::ast;
use syntax::ast::Mod;
use syntax::attr;
use syntax::parse::token;
use syntax::parse::token::InternedString;
use syntax::owned_slice::OwnedSlice;

fn get_rustspec_extern_crate() -> ast::ViewItem {
    ast::ViewItem {
        node: ast::ViewItemExternCrate(token::str_to_ident("rustspec"),
        Some((token::intern_and_get_ident("rustspec"), ast::CookedStr)),
        ast::DUMMY_NODE_ID),
        attrs: vec!(
            attr::mk_attr_outer(attr::mk_attr_id(), attr::mk_list_item(
                InternedString::new("phase"),
                vec!(
                    attr::mk_word_item(InternedString::new("plugin")),
                    attr::mk_word_item(InternedString::new("link"))
                    )
                ))
            ),
        vis: ast::Inherited,
        span: DUMMY_SP
    }
}

fn get_rustspec_assertions_use() -> ast::ViewItem {
    let prelude_path = ast::Path {
        span: DUMMY_SP,
        global: false,
        segments: vec!(
            ast::PathSegment {
                identifier: token::str_to_ident("rustspec"),
                lifetimes: Vec::new(),
                types: OwnedSlice::empty(),
            }
        ),
    };

    let vp = box(GC) codemap::dummy_spanned(ast::ViewPathGlob(prelude_path, ast::DUMMY_NODE_ID));

    ast::ViewItem {
        node: ast::ViewItemUse(vp),
        attrs: Vec::new(),
        vis: ast::Inherited,
        span: DUMMY_SP,
    }
}

pub struct TestContextNode {
    name: String,
    before: Option<Gc<syntax::ast::Block>>,
    children: Vec<Box<TestNode>>
}

impl TestContextNode {
    pub fn new(name: String,
               before: Option<Gc<syntax::ast::Block>>,
               children: Vec<Box<TestNode>>
              ) -> Box<TestContextNode> {
        box TestContextNode { name: name, children: children, before: before }
    }
}

impl TestNode for TestContextNode {
    fn to_item(&self, cx: &mut ExtCtxt, before_blocks: &mut Vec<Gc<syntax::ast::Block>>) -> Gc<ast::Item> {
        if self.before.is_some() {
            before_blocks.push(self.before.unwrap());
        }

        let children_items = self.children.iter().map(|i| i.to_item(cx, before_blocks)).collect::<Vec<Gc<syntax::ast::Item>>>();

        if self.before.is_some() {
            before_blocks.pop();
        }

        box(GC) ast::Item {
            ident: cx.ident_of(self.get_name().as_slice()),
            attrs: vec![],
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemMod(Mod {
                inner: DUMMY_SP,
                view_items: vec![
                    get_rustspec_extern_crate(),
                    get_rustspec_assertions_use()
                ],
                items: children_items
            }),
            vis: ast::Inherited,
            span: DUMMY_SP,
        }
    }

    fn get_name(&self) -> String {
        self.name.replace(" ", "_").replace("#", "")
    }
}
