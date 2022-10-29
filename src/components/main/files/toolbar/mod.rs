use dioxus::{prelude::*, events::MouseEvent};
use dioxus_heroicons::outline::Shape;

use crate::components::ui_kit::icon_button::IconButton;

#[derive(Props)]
pub struct Props<'a> {
    on_new_folder: EventHandler<'a, MouseEvent>,
    on_show_upload: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn Toolbar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            id: "toolbar",
            div {
                id: "controls",
                IconButton {
                    icon: Shape::Archive,
                    state: crate::components::ui_kit::icon_button::State::Secondary,
                    on_pressed: move |_| {}
                },
                IconButton {
                    icon: Shape::FolderAdd,
                    state: crate::components::ui_kit::icon_button::State::Secondary,
                    on_pressed: move |e| cx.props.on_new_folder.call(e)
                },
                IconButton {
                    icon: Shape::Upload,
                    on_pressed: move |e| cx.props.on_show_upload.call(e)
                }
            },
            div {
                id: "close",
                IconButton {
                    on_pressed: move |_| {
                        use_router(&cx).push_route("/main", None, None);
                    },
                    icon: Shape::X
                }
            }
        },
    })
}
