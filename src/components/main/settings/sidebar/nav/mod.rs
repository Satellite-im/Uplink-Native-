use dioxus::prelude::*;

use crate::components::ui_kit::button::Button;

#[derive(PartialEq, Eq)]
pub enum NavEvent {
    General,
    Privacy,
    AudioVideo,
    Extensions,
    Developer,
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    text: String,
    active: bool,
    disabled: bool,
    on_pressed: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn NavButton<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let class = if cx.props.active {
        "active"
    } else {
        "inactive"
    };

    cx.render(rsx!(
        div {
            class: "nav-button {class}",
            Button {
                on_pressed: move |_| cx.props.on_pressed.call(()),
                disabled: cx.props.disabled,
                text: cx.props.text.clone()
            }
        }
    ))
}

#[derive(Props)]
pub struct Props<'a> {
    on_pressed: EventHandler<'a, NavEvent>,
}

#[allow(non_snake_case)]
pub fn Nav<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let active_item = use_state(&cx, || NavEvent::Developer);

    cx.render(rsx! {
        div {
            class: "nav",
            NavButton {
                text: String::from("General"),
                active: NavEvent::General.eq(active_item),
                disabled: true,
                on_pressed: |_| {
                    active_item.set(NavEvent::General);
                    let _ = cx.props.on_pressed.call(NavEvent::General);
                }
            },
            NavButton {
                text: String::from("Privacy"),
                active: NavEvent::Privacy.eq(active_item),
                disabled: true,
                on_pressed: |_| {
                    active_item.set(NavEvent::Privacy);
                    let _ = cx.props.on_pressed.call(NavEvent::Privacy);
                }
            },
            NavButton {
                text: String::from("AudioVideo"),
                active: NavEvent::AudioVideo.eq(active_item),
                disabled: true,
                on_pressed: |_| {
                    active_item.set(NavEvent::AudioVideo);
                    let _ = cx.props.on_pressed.call(NavEvent::AudioVideo);
                }
            },
            NavButton {
                text: String::from("Extensions"),
                active: NavEvent::Extensions.eq(active_item),
                disabled: true,
                on_pressed: |_| {
                    active_item.set(NavEvent::Extensions);
                    let _ = cx.props.on_pressed.call(NavEvent::Extensions);
                }
            },
            NavButton {
                text: String::from("Developer"),
                active: NavEvent::Developer.eq(active_item),
                disabled: false,
                on_pressed: |_| {
                    active_item.set(NavEvent::Developer);
                    let _ = cx.props.on_pressed.call(NavEvent::Developer);
                }
            }
        }
    })
}