#![crate_name="rustspec"]
#![crate_type="dylib"]
#![feature(macro_rules, phase, plugin_registrar)]

extern crate syntax;
extern crate rustc;

use macro_result::MacroResult;
use test_context_node::TestContextNode;
use test_case_node::TestCaseNode;
use test_node::TestNode;

use std::gc::{Gc};
use rustc::plugin::Registry;
use syntax::ext::base::{ExtCtxt, MacResult};
use syntax::ext::quote::rt::ToTokens;
use syntax::codemap::Span;
use syntax::ast;
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
    token == token::LBRACE || token == token::RBRACE ||
        token == token::LPAREN || token == token::RPAREN ||
        token == token::COMMA || token == token::SEMI
}

fn extract_test_node_data(parser: &mut Parser) -> (String, Gc<ast::Block>) {
    parser.bump(); // skip  (
    let (name, _) = parser.parse_str();
    parser.bump(); // skip ,
    let block = parser.parse_block();
    (name.get().to_string(), block)
}

fn parse_test_node(parser: &mut Parser) -> Box<TestCaseNode> {
    let mut should_fail = false;
    let mut should_be_ignored = false;

    if parser.token == token::DOT {
        parser.bump();
        let ident = parser.parse_ident();
        let token_str = ident.as_str();
        should_fail = token_str == "fails";
        should_be_ignored = token_str == "ignores";
    }

    let (name, block) = extract_test_node_data(parser);
    TestCaseNode::new(name, block, should_fail, should_be_ignored)
}

fn parse_node(cx: &mut ExtCtxt, parser: &mut Parser) -> (Option<Gc<syntax::ast::Block>>, Vec<Box<TestNode>>) {
    let mut nodes: Vec<Box<TestNode>> = Vec::new();
    let mut before_block = None;

    while parser.token != token::EOF {
        if is_skippable(parser.token.clone()) {
            parser.bump();
            continue;
        }

        let ident = parser.parse_ident();
        let token_str = ident.as_str();

        match token_str {
            "before" => {
                if before_block.is_some() {
                    fail!("More than one before blocks found in the same context.");
                }

                parser.bump(); // skip  (
                before_block = Some(parser.parse_block());
            },

            "when" | "context" | "describe" => {
                parser.bump(); // skip  (
                let (name, _) = parser.parse_str();
                parser.bump(); // skip ,
                let block_tokens = parser.parse_block().to_tokens(cx);
                let mut block_parser = tts_to_parser(cx.parse_sess(), block_tokens, cx.cfg());
                let (b, children) = parse_node(cx, &mut block_parser);
                nodes.push(TestContextNode::new(name.get().to_string(), b, children));
            },

            "it" => {
                nodes.push(parse_test_node(parser));
            },

            other =>  {
                let span = parser.span;
                parser.span_fatal(span, format!("Unexpected {}", other).as_slice());
            }
        }
    }

    (before_block, nodes)
}

pub fn macro_scenario(cx: &mut ExtCtxt, _: Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    let mut parser = cx.new_parser_from_tts(tts);
    // TODO MOD THIS
    let _ = parser.parse_str();
    parser.bump();
    let block_tokens = parser.parse_block().to_tokens(cx);
    let mut block_parser = tts_to_parser(cx.parse_sess(), block_tokens, cx.cfg());
    let (_, root_nodes) = parse_node(cx, &mut block_parser);
    // TODO before for the scenario?
    let items = root_nodes.iter().map(|i| i.to_item(cx, &mut vec![])).collect();
    MacroResult::new(items)
}

