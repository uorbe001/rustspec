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

fn get_rustspec_extern_crate() -> P<ast::Item> {
    P(ast::Item {
        node: ast::ItemExternCrate(Some(token::intern("rustspec"))),
        id: ast::DUMMY_NODE_ID,
        ident: token::str_to_ident("rustspec"),

        attrs: vec![],
        vis: ast::Inherited,
        span: DUMMY_SP
    })
}

fn get_rustspec_assertions_use() -> P<ast::Item> {
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

    let vp = P(codemap::dummy_spanned(ast::ViewPathGlob(prelude_path)));

    P(ast::Item {
        node: ast::ItemUse(vp),
        attrs: Vec::new(),
        vis: ast::Inherited,
        span: DUMMY_SP,
        id: ast::DUMMY_NODE_ID,
        ident: token::str_to_ident("rustspec")
    })
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
        Box::new(TestContextNode { name: name, children: children, before: before })
    }
}

impl TestNode for TestContextNode {
    fn to_item(&self, cx: &mut ExtCtxt, before_blocks: &mut Vec<P<ast::Block>>) -> P<ast::Item> {
        self.before.as_ref().and_then(|before| {
            before_blocks.push(before.clone());
            None::<P<ast::Block>>
        });

        let mut children_items = self.children.iter().map(|i| i.to_item(cx, before_blocks)).collect::<Vec<P<ast::Item>>>();
        children_items.push(get_rustspec_extern_crate());
        children_items.push(get_rustspec_assertions_use());

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
