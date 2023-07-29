use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageChangelog() -> impl IntoView {
    view! { 
        <H1>"Changelog"</H1>

        <H2>"0.1.0"</H2>

        <P>"Initial release."</P>

        <H3>"Added utilities:"</H3>
        <ul>
            <li>"Callback types"</li>
            <li>"OptionalMaybeSignal type"</li>
            <li>"Global event listener contexts"</li>
        </ul>

        <H3>"Added components:"</H3>
        <ul>
            <li>"Root component"</li>
            <li>"Skeleton component and styles"</li>
            <li>"Stack component and styles"</li>
            <li>"Grid component and styles"</li>
            <li>"Separator component and styles"</li>
            <li>"Tab components and styles"</li>
            <li>"Collapsible components and styles"</li>
            <li>"AppBar components and styles"</li>
            <li>"Drawer components and styles"</li>
            <li>"Button component and styles"</li>
            <li>"Input component and styles"</li>
            <li>"Date selector component and styles"</li>
            <li>"Slider component and styles"</li>
            <li>"Select component and styles"</li>
            <li>"Toggle component and styles"</li>
            <li>"Alert component and styles"</li>
            <li>"Toast component and styles"</li>
            <li>"Modal components and styles"</li>
            <li>"Progress component and styles"</li>
            <li>"Popover component and styles"</li>
            <li>"Chip component and styles"</li>
            <li>"Icon component and styles"</li>
            <li>"Link component and styles"</li>
            <li>"Anchor component and styles"</li>
            <li>"Typography components and styles"</li>
            <li>"Transition components and styles"</li>
        </ul>
    }
}
