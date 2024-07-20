use pulldown_cmark::Event::Html;
use pulldown_cmark::{html, Options, Parser};
use serde_json::Value::String;
use web_sys::Node;
use yew::prelude::*;

/// 解析 Markdown，转为 HTML
pub fn convert_markdown_to_html(mk: String) -> Html {
    let options = Options::all();
    let parser = Parser::new_ext(&mk, options);

    let mut mk_html = String::new();
    html::push_html(&mut mk_html, parser);

    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&mk_html);
    
    let node: Node = div.into();
    Html::VRef(node);
}
