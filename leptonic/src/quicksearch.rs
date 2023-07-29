use crate::prelude::*;
use leptos::*;

#[component]
pub fn Quicksearch<T, IV>(
    trigger: T,
    #[prop(into)] query: Callback<String, Vec<QuicksearchOption>>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView
where
    T: Fn(WriteSignal<bool>) -> IV + 'static,
    IV: IntoView + 'static,
{
    let (show_modal, set_show_modal) = create_signal(false);
    view! {
        <leptonic-quicksearch id=id class=class style=style>
            { trigger(set_show_modal) }
            <QuicksearchModal
                show_when=show_modal
                query=query
                on_cancel=move || set_show_modal.set(false)
            />
        </leptonic-quicksearch>
    }
}

#[component]
pub fn QuicksearchTrigger(
    #[prop(into)] set_quicksearch: WriteSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-quicksearch-trigger id=id class=class style=style on:click=move |_| set_quicksearch.set(true)>
            { children() }
        </leptonic-quicksearch-trigger>
    }
}

#[derive(Debug, Clone)]
pub struct QuicksearchOption {
    pub view: Callback<(), View>,
    pub on_select: Callback<()>,
}

#[component]
fn QuicksearchModal<C>(
    #[prop(into)] show_when: Signal<bool>,
    #[prop(into)] query: Callback<String, Vec<QuicksearchOption>>,
    on_cancel: C,
) -> impl IntoView
where
    C: Fn() + Copy + 'static,
{
    let (input, set_input) = create_signal("".to_owned());

    let options = move || query.call(input.get());

    let g_keyboard_event: GlobalKeyboardEvent = expect_context::<GlobalKeyboardEvent>();
    create_effect(move |_old| {
        if let Some(e) = g_keyboard_event.read_signal.get() {
            if show_when.get_untracked() && e.key().as_str() == "Escape" {
                on_cancel();
            }
        }
    });

    view! {
        <Modal show_when=show_when class="quicksearch-modal">
            <ModalHeader>
                <Input
                    get=input
                    set=move |v| set_input.set(v)
                    label="Search"
                    class="search-input"
                    should_be_focused=show_when
                    prepend=view! {""}.into_view()
                />
            </ModalHeader>
            <ModalBody>
                <leptonic-quicksearch-results>
                    { move || options().into_iter().map(|option| view! {
                        <leptonic-quicksearch-result on:click=move |_| {
                                option.on_select.call(());
                                on_cancel();
                            }>
                            { option.view.call(()) }
                        </leptonic-quicksearch-result>
                    }).collect_view() }
                </leptonic-quicksearch-results>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| (on_cancel)() color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
