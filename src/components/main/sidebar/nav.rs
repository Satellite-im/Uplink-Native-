use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;

use crate::components::{ui_kit::{icon_button::{IconButton, self}}};

pub enum NavEvent {
    Home,
    Files,
    Friends,
    Profile,
}

#[derive(Props)]
pub struct Props<'a> {
    onclick: EventHandler<'a, NavEvent>,
}

#[allow(non_snake_case)]
pub fn Nav<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    global_css! {"
        .nav {
            display: inline-flex;
            flex-direction: row;
            justify-content: space-evenly;
            position: absolute;
            bottom: 1rem;
            left: 0;
            right: 0;

            .button {
                svg {
                    stroke: var(--theme-text-bright);
                }
            }
            .button-secondary {
                background: none;
                svg {
                    stroke: var(--theme-text-muted);
                }
            }
        }
    "}
    cx.render(rsx!{
        div {
            class: "nav",
            IconButton {
                onclick: move |_| {
                    let _ = &cx.props.onclick.call(NavEvent::Home);
                },
                icon: Shape::Home
            },
            IconButton {
                onclick: move |_| {
                    let _ = &cx.props.onclick.call(NavEvent::Files);
                },
                state: icon_button::State::Secondary,
                icon: Shape::Folder
            },
            IconButton {
                onclick: move |_| {
                    let _ = &cx.props.onclick.call(NavEvent::Friends);
                },
                state: icon_button::State::Secondary,
                icon: Shape::Users
            },
            IconButton {
                onclick: move |_| {
                    let _ = &cx.props.onclick.call(NavEvent::Profile);
                },
                state: icon_button::State::Secondary,
                icon: Shape::UserCircle
            },
        }
    })
}