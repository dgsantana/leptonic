use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageInput() -> impl IntoView {
    let (text, set_text) = create_signal("text".to_owned());
    let (password, set_password) = create_signal("secret".to_owned());
    let (number, set_number) = create_signal("number".to_owned());
    let (label_input, set_label_input) = create_signal("".to_owned());

    view! { 
        <H1>"Inputs"</H1>

        <P>"Creating an input is as simple as doing the following"</P>
        <Code>
            {indoc!(r#"
                let (text, set_text) = create_signal("text".to_owned());
                view! { 
                    <Input get=text set=set_text/>
                }
            "#)}
        </Code>

        <Input get=text set=move |v| set_text.set(v)/>

        <H2>"Types"</H2>

        <P>"You can use the " <Code inline=true>"InputType"</Code> " enum, to either create a text, password or number input."</P>

        <Code>
            {indoc!(r#"
                let (password, set_password) = create_signal("secret".to_owned());
                view! { 
                    <Input ty=InputType::Password get=text set=set_text/>
                }
            "#)}
        </Code>

        <Input get=password set=move |v| set_password.set(v) ty=InputType::Password/>

        <Input get=number set=move |v| set_number.set(v) ty=InputType::Number/>

        <H2>"Label"</H2>

        <P>"You can supply a label to the input. It is shown as the inputs placeholder when the input is empty."</P>

        <Code>
            {indoc!(r#"
                let (text, set_text) = create_signal("".to_owned());
                view! { 
                    <Input get=text set=set_text label="This is my label"/>
                    <Button
                        variant=ButtonVariant::Flat
                        size=ButtonSize::Small
                        on_click=move |_| set_text.set("".to_owned())>
                        "Clear input"
                    </Button>
                }
            "#)}
        </Code>

        <Input get=label_input set=move |v| set_label_input.set(v) label="This is my label"/>
        <Button variant=ButtonVariant::Flat size=ButtonSize::Small on_click=move |_| set_label_input.set("".to_owned())>"Clear input"</Button>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --input-padding
                --input-color
                --input-background-color
                --input-border:
                --input-border-bottom
                --input-border-radius
                --input-min-height
                --input-focused-border-color
            "#)}
        </Code>
    }
}
