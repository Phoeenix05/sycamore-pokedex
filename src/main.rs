mod pages;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| sycamore::view! { cx, pages::RouterView { } });
}
