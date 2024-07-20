use yew::prelude::*;

use crate::components::{card::Card, container::AppContext};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit("找不到该页面".into());

    html! {
        <Card title={"找不到该页面"}>
            <p>{"尝试换个地址？"}</p>
        </Card>
    }
}
