extern crate syntax;

use test_node::TestNode;

use syntax::ext::base::ExtCtxt;
use syntax::codemap;
use syntax::codemap::DUMMY_SP;
use syntax::ast;
use syntax::ptr::P;
use syntax::ast::Mod;
use syntax::attr;
use syntax::parse::token;
use syntax::parse::token::InternedString;

fn get_rustspec_extern_crate() -> ast::ViewItem {
    ast::ViewItem {
        node: ast::ViewItemExternCrate(token::str_to_ident("rustspec"),
        Some((token::intern_and_get_ident("rustspec"), ast::CookedStr)),
        ast::DUMMY_NODE_ID),
        attrs: vec![
            attr::mk_attr_outer(attr::mk_attr_id(), attr::mk_list_item(
                InternedString::new("phase"),
                vec!(
                    attr::mk_word_item(InternedString::new("plugin")),
                    attr::mk_word_item(InternedString::new("link"))
                    )
                )
            ),
        ],
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
                parameters: ast::PathParameters::none()
            }
        ),
    };

    let vp = P(codemap::dummy_spanned(ast::ViewPathGlob(prelude_path, ast::DUMMY_NODE_ID)));

    ast::ViewItem {
        node: ast::ViewItemUse(vp),
        attrs: Vec::new(),
        vis: ast::Inherited,
        span: DUMMY_SP,
    }
}

pub struct TestContextNode {
    name: String,
    before: Option<P<ast::Block>>,
    children: Vec<Box<TestNode + 'static>>
}

impl TestContextNode {
    pub fn new(name: String,
               before: Option<P<ast::Block>>,
               children: Vec<Box<TestNode + 'static>>
              ) -> Box<TestContextNode> {
        box TestContextNode { name: name, children: children, before: before }
    }
}

impl TestNode for TestContextNode {
    fn to_item(&self, cx: &mut ExtCtxt, before_blocks: &mut Vec<P<ast::Block>>) -> P<ast::Item> {
        self.before.as_ref().and_then(|before| {
            before_blocks.push(before.clone());
            None::<P<ast::Block>>
        });

        let children_items = self.children.iter().map(|i| i.to_item(cx, before_blocks)).collect::<Vec<P<ast::Item>>>();

        if self.before.is_some() {
            before_blocks.pop();
        }

        let mut attributes = vec![];

        attributes.push(attr::mk_attr_outer(attr::mk_attr_id(), attr::mk_list_item(
                InternedString::new("allow"),
                vec!(attr::mk_word_item(InternedString::new("unused_attribute")))
            )
        ));

        attributes.push(attr::mk_attr_outer(attr::mk_attr_id(), attr::mk_list_item(
                InternedString::new("allow"),
                vec!(attr::mk_word_item(InternedString::new("non_snake_case")))
            )
        ));

        P(ast::Item {
            ident: cx.ident_of(self.get_name().as_slice()),
            attrs: attributes,
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
        })
    }

    fn get_name(&self) -> String {
        self.name.replace(" ", "_").replace("#", "")
    }
}
