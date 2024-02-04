use leptos_memo_test::components_with_logging::App;
// use leptos_memo_test::components::App;

use leptos::mount_to_body;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    mount_to_body(App)
}
