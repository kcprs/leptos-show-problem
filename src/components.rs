use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn ItemPane(item: Item) -> impl IntoView {
    let selected_item = expect_context::<SelectedItem>().0;
    let items = expect_context::<ExistingItems>().0;

    let is_this_selected = {
        move || {
            selected_item.with(|x| {
                if let Some(selected_item) = x {
                    &item == selected_item
                } else {
                    false
                }
            })
        }
    };

    let set_this_selected = move |_| {
        selected_item.set(Some(item));
    };

    let close_this = move |e: MouseEvent| {
        e.stop_propagation();
        items.update(|v| v.retain(|i| *i != item));
        if is_this_selected() {
            selected_item.set(None);
        }
    };

    view! {
        <div class:selected=is_this_selected on:click=set_this_selected>
            <p>{move || format!("Item {}", item)}</p>
            <button on:click=close_this>close</button>
        </div>
    }
}

#[component]
fn ItemsView() -> impl IntoView {
    let items = expect_context::<ExistingItems>().0;

    view! {
        <For
            each=move || items.get()
            key=|item| *item
            children=move |item| {
                view! { <ItemPane item=item/> }
            }
        />
    }
}

#[component]
fn SelectionView() -> impl IntoView {
    // struct SelectedItem(RwSignal<Option<Item>>);
    let selected_item = expect_context::<SelectedItem>().0.read_only();

    view! {
        <Show
            when=move || selected_item.get().is_some()
            fallback=|| {
                view! { "No item selected" }
            }
        >

            "Item "
            {move || selected_item.get().unwrap()}
            " is selected"
        </Show>
    }
}

type Item = usize;

#[derive(Copy, Clone)]
struct ExistingItems(RwSignal<Vec<Item>>);

#[derive(Copy, Clone)]
struct SelectedItem(RwSignal<Option<Item>>);

#[component]
pub fn App() -> impl IntoView {
    let mut items = vec![];
    [0, 1, 2, 3].iter().for_each(|&i| items.push(i));

    provide_context(SelectedItem(create_rw_signal(Some(items[0]))));
    provide_context(ExistingItems(create_rw_signal(items)));

    view! {
        <main>
            <SelectionView/>
            <ItemsView/>
        </main>
    }
}
