use leptos::*;
use leptos_use::on_click_outside;

#[component]
pub fn App() -> impl IntoView {
    let (show, set_show) = create_signal(false);
    let toggle = move |_| match show.get_untracked() {
        true => {
            tracing::info!("hide <Test>");
            set_show.set(false);
        }
        false => {
            tracing::info!("show <Test>");
            set_show.set(true)
        }
    };
    view! {
        <button on:click=toggle>"Toggle component"</button>
        { move || show.get().then(|| view! { <Test/> }) }
    }
}

#[component]
pub fn Test() -> impl IntoView {
    tracing::info!("initialize <Test>");
    let el = create_node_ref();
    let on_click_outside_dispose =
        on_click_outside(el, move |_e| tracing::info!("clicked outside"));
    on_cleanup(move || {
        tracing::info!("cleanup <Test>");
        on_click_outside_dispose();
    });
    view! {
        <div node_ref=el>"Element with on_click_outside listener"</div>
        <div>"Sibling"</div>
    }
}
