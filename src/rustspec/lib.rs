#![crate_name="rustspec"]
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, collections, core, convert)]

extern crate syntax;
extern crate core;
extern crate rustc;
extern crate rustspec_assertions;

pub use rustspec_assertions::{expect, eq, be_gt, be_ge, be_lt, be_le, contain, be_false, be_true, be_some, be_none};

use macro_result::MacroResult;
use test_context_node::TestContextNode;
use test_case_node::TestCaseNode;
use test_node::TestNode;

use self::core::ops::Deref;
use rustc::plugin::Registry;
use syntax::ext::base::{ExtCtxt, MacResult};
use syntax::ext::quote::rt::ToTokens;
use syntax::codemap::Span;
use syntax::ast;
use syntax::ptr::P;
use syntax::parse::{token, tts_to_parser};
use syntax::parse::parser::Parser;

mod macro_result;
mod test_context_node;
mod test_case_node;
mod test_node;

#[plugin_registrar]
pub fn plugin_registrar(registry: &mut Registry) {
    registry.register_macro("scenario", macro_scenario);
}

fn is_skippable(token: syntax::parse::token::Token) -> bool {
    token == token::OpenDelim(token::Brace) ||
        token == token::CloseDelim(token::Brace) ||
        token == token::OpenDelim(token::Paren) ||
        token == token::CloseDelim(token::Paren) ||
        token == token::Comma || token == token::Semi
}

#[allow(unused_must_use)]
fn extract_test_node_data(parser: &mut Parser) -> (String, P<ast::Block>) {
    parser.bump(); // skip  (
    let (name, _) = parser.parse_str().ok().unwrap();
    parser.bump(); // skip ,
    let block = parser.parse_block().ok().unwrap();
    (name.deref().to_string(), block)
}

#[allow(unused_must_use)]
fn parse_test_node(parser: &mut Parser) -> Box<TestCaseNode> {
    let mut should_fail = false;
    let mut should_be_ignored = false;

    if parser.token == token::Dot {
        parser.bump();
        let ident = parser.parse_ident().ok().unwrap();
        let token_str = ident.as_str();
        should_fail = token_str == "fails";
        should_be_ignored = token_str == "ignores";
    }

    let (name, block) = extract_test_node_data(parser);
    TestCaseNode::new(name, block, should_fail, should_be_ignored)
}

#[allow(unused_must_use)]
fn parse_node(cx: &mut ExtCtxt, parser: &mut Parser) -> (Option<P<ast::Block>>, Vec<Box<TestNode + 'static>>) {
    let mut nodes: Vec<Box<TestNode>> = Vec::new();
    let mut before_block = None;

    while parser.token != token::Eof {
        if is_skippable(parser.token.clone()) {
            parser.bump();
            continue;
        }

        let ident = parser.parse_ident().ok().unwrap();
        let token_str = ident.as_str();

        match token_str {
            "before" => {
                if before_block.is_some() {
                    panic!("More than one before blocks found in the same context.");
                }

                parser.bump(); // skip  (
                before_block = Some(parser.parse_block().ok().unwrap());
            },

            "when" | "context" | "describe" => {
                parser.bump(); // skip  (
                let (name, _) = parser.parse_str().ok().unwrap();
                parser.bump(); // skip ,
                let block_tokens = parser.parse_block().ok().unwrap().to_tokens(cx);
                let mut block_parser = tts_to_parser(cx.parse_sess(), block_tokens, cx.cfg());
                let (b, children) = parse_node(cx, &mut block_parser);

                let before = if b.is_some() {
                    Some(P(b.unwrap().deref().clone()))
                } else { None };

                nodes.push(TestContextNode::new(
                        name.deref().to_string(),
                        before,
                        children
                ));
            },

            "it" => {
                nodes.push(parse_test_node(parser));
            },

            other =>  {
                let span = parser.span;
                parser.span_fatal(span, format!("Unexpected {}", other).as_ref());
            }
        }
    }

    (before_block, nodes)
}

#[allow(unused_must_use)]
pub fn macro_scenario(cx: &mut ExtCtxt, _: Span, tts: &[ast::TokenTree]) -> Box<MacResult + 'static> {
    let mut parser = cx.new_parser_from_tts(tts);

    let (name, _) = parser.parse_str().ok().unwrap();
    parser.bump();
    let block_tokens = parser.parse_block().ok().unwrap().to_tokens(cx);
    let mut block_parser = tts_to_parser(cx.parse_sess(), block_tokens, cx.cfg());
    let (before, children) = parse_node(cx, &mut block_parser);
    let node = TestContextNode::new(name.deref().to_string(), before, children);
    MacroResult::new(vec![node.to_item(cx, &mut vec![])])
}

