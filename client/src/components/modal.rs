use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub children: Children,
    #[prop_or(html! {})]
    pub footer: Html,
    #[prop_or(true)]
    pub open: bool,
}

// 在 Rust 里调用 JS 函数
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
}

/// 一个模态的弹窗
#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    // 如果同时出现多个 Modal，由于 id 相同，就无法正常关闭，所以需要给每个 Modal 随机的 id
    let modal_id = format!("modal_{}", random());

    html! {
        <div class="modal">
            <input id={modal_id.clone()} type="checkbox" checked={props.open}/>
            <label for={modal_id.clone()} class="overlay"></label>
            <article>
                <header>
                    <h3>{ &props.title }</h3>
                    <label for={modal_id.clone()} class="close">{ "✖" }</label>
                </header>
                <section class="children">
                { for props.children.iter() }
                </section>
                <footer>
                { props.footer.clone() }
                </footer>
            </article>
        </div>
    }
}