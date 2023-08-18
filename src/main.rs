use reqwasm::http::Request;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

async fn fetch_data(id: &str) -> Result<String, reqwasm::Error> {
    let url = format!("https://api.mangadex.org/manga/{id}/feed");
    let resp = Request::get(&url).send().await?;
    let body = resp.text().await?;
    Ok(body)
}

#[component]
async fn Data<G: Html>(cx: Scope<'_>) -> View<G> {
    let data = fetch_data("bd6d0982-0091-4945-ad70-c028ed3c0917")
        .await
        .unwrap_or_default();

    view! { cx,
        pre { (data) }
    }
}

#[component]
async fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    // let state = create_signal(cx, 0);
    // let increment = |_| state.set(*state.get() + 1);
    // let decrement = |_| state.set(*state.get() - 1);

    view! { cx,
        // p { "Hallo, World! " (state.get()) }
        // button(on:click=increment) { "+" }
        // button(on:click=decrement) { "-" }
        div {
            p(class="border-2 border-red-500 rounded-lg") { "Manga data" }
            Suspense(fallback=view! { cx, "Loading..." }) {
                Data { }
            }
        }
    }
}

fn main() {
    sycamore::render(App);
}
