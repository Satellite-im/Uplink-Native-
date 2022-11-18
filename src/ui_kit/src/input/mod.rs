use dioxus::{
    events::{FormEvent, KeyboardData, MouseEvent},
    prelude::*,
};
use dioxus_elements::KeyCode;

use crate::context_menu::{ContextItem, ContextMenu};

#[derive(PartialEq, Eq)]
pub enum State {
    Success,
    Danger,
}

#[derive(Props)]
pub struct Props<'a> {
    placeholder: String,
    on_change: EventHandler<'a, FormEvent>,
    // on_enter: EventHandler<'a, ()>,
    #[props(optional)]
    value: Option<String>,
}

#[allow(non_snake_case)]
pub fn Input<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    cx.render(rsx! {
            span {
                id: "TODO-input-input",
                ContextMenu {
                    parent: String::from("TODO-input-input"),
                    items: cx.render(rsx! {
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Paste"),
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Select All"),
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Copy"),
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Clear"),
                        },
                    })
                },
        cx.render(match &cx.props.value {
            Some(value) => rsx!(input {
                class: "input",
                placeholder: "{cx.props.placeholder}",
                value: "{value}",
                oninput: |evt| cx.props.on_change.call(evt),
                // onkeyup: move |evt| {
                //     if evt.key_code == KeyCode::Enter {
                //         cx.props.on_enter.call(())
                //     }
                // }
            }),
            None => rsx! {
                input {
                    class: "input",
                    placeholder: "{cx.props.placeholder}",
                    oninput: |evt| cx.props.on_change.call(evt),
                    // onkeyup:move |evt| {
                    //     if evt.key_code == KeyCode::Enter {
                    //         cx.props.on_enter.call(())
                    //     }
                    // }
                }
            },
        }),}
    })
}
