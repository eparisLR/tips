use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use crate::entity::tips::Tips;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    greeting: String,
    tips: Vec<Tips>
}

#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &IndexPageStateRx) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
        (View::new_fragment(
            state.tips.get()
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let item = item.clone();
                    view! { cx,
                        p(class = "todo-item") {
                            (item.title)
                        }
                    }
                })
                .collect(),
        ))
        a(href = "about", id = "about-link") { "About!" }
    }
}

#[engine_only_fn]
fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    let client = reqwest::Client::new();
    let tips = client.get("http://localhost:3000/tips").send().await;
    IndexPageState {
        greeting: "Hello World!".to_string(),
        tips: tips.expect("Failed to get response").json::<Vec<Tips>>().await.expect("Failed to get tips")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}