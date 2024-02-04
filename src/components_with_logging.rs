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
        logging::log!("Setting selected item to {}", item);
        selected_item.set(Some(item));
    };

    let close_this = move |e: MouseEvent| {
        logging::log!("Closing item {}", item);
        // Stop event from bubbling up to parent elements.
        // Why bubbling needs to be stopped in this particular case:
        //  1. Click event received in "close" button
        //  2. Button's onclick executed -> current item deselected and closed
        //  3. Click event bubbles up to button's parent, which is ItemPane
        //  4. ItemPanes's onclick executed -> current item selected again
        e.stop_propagation();

        items.update(|v| v.retain(|i| *i != item));
        if is_this_selected() {
            logging::log!("Setting selected item to None");
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
    let selected_item = expect_context::<SelectedItem>().0.read_only();

    view! {
        <Show
            when=move || {
                logging::log!("Checking if item is selected...");
                let item = selected_item.get();
                let is_some = item.is_some();
                if is_some {
                    logging::log!("Selected item is Some({})", item.unwrap());
                } else {
                    logging::log!("Selected item is None");
                }
                is_some
            }

            fallback=|| {
                logging::log!("Updating displayed item...");
                logging::log!("Set displayed item to None");
                view! { "No item selected" }
            }
        >

            "Item "
            {move || {
                logging::log!("Updating displayed item...");
                let selected_item = selected_item.get().unwrap();
                logging::log!("Set displayed item to {}", selected_item);
                selected_item
            }}

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
