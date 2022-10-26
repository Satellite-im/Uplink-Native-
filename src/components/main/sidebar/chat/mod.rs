use crate::{
    components::ui_kit::skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    state::ConversationInfo,
    Account, Messaging, LANGUAGE, STATE,
};
use dioxus::prelude::*;
use futures::stream::StreamExt;
use uuid::Uuid;
use warp::raygun::{MessageEventKind, MessageOptions, RayGun, RayGunStream};

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    conversation_info: ConversationInfo,
    messaging: Messaging,
    on_pressed: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Chat<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let state = use_atom_ref(&cx, STATE).clone();
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let unread_count = use_state(&cx, || 0_usize).clone();
    let unread_count2 = unread_count.clone();

    let mut rg = cx.props.messaging.clone();
    let mp = cx.props.account.clone();

    let ident = mp
        .read()
        .get_own_identity()
        .expect("Unexpected error <temp>");

    let username = cx
        .props
        .conversation_info
        .conversation
        .recipients()
        .iter()
        //filters out our own did key in the iter
        .filter(|did| ident.did_key().ne(did))
        //tries get_identity so if it returns Option::Some it would be the map item, otherwise its filtered out
        .filter_map(|did| mp.read().get_identity(did.clone().into()).ok())
        //flatted the nested iterators
        .flatten()
        .map(|i| i.username())
        .last()
        .unwrap_or_default();

    let show_skeleton = username.is_empty();

    let is_active = state
        .read()
        .current_chat
        .map(|x| x == cx.props.conversation_info.conversation.id())
        .unwrap_or(false);

    let active = if is_active { "active" } else { "none" };

    use_future(
        &cx,
        &cx.props.conversation_info.clone(),
        |conversation_info| async move {
            if is_active {
                unread_count.set(0);
                // very important: don't open two message streams - if this is the active chat, the messages Element will read the stream and this
                // chat component shouldn't.
                return;
            }
            let messages = rg
                .get_messages(
                    conversation_info.conversation.id(),
                    MessageOptions::default(),
                )
                .await
                .unwrap_or_default();

            let num_unread_messages = match conversation_info.last_msg_read {
                // assumes the most recent messages appear first in the list
                Some(id) => messages.iter().take_while(|x| x.id() != id).count(),
                None => messages.len(),
            };

            if *unread_count.current() != num_unread_messages {
                unread_count.set(num_unread_messages);
            }

            let mut stream = loop {
                match rg
                    .get_conversation_stream(conversation_info.conversation.id())
                    .await
                {
                    Ok(stream) => break stream,
                    Err(e) => match &e {
                        warp::error::Error::RayGunExtensionUnavailable => {
                            //Give sometime for everything in the background to fully line up
                            //Note, if this error still happens, it means there is an fatal error
                            //      in the background
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        }
                        _ => {
                            // todo: properly report this error
                            // eprintln!("failed to get_conversation_stream: {}", e);
                            tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                        }
                    },
                }
            };

            while let Some(event) = stream.next().await {
                if let MessageEventKind::MessageReceived { .. } = event {
                    unread_count.modify(|x| x + 1);
                }
            }
        },
    );

    if show_skeleton {
        cx.render(rsx! {
            div {
                class: "chat {active}",
                onclick: move |_| cx.props.on_pressed.call(()),
                PFPSkeleton {},
                div {
                    class: "who",
                    InlineSkeleton {},
                    InlineSkeleton {}
                }
            }
        })
    } else {
        cx.render(rsx! {
            div {
                class: "chat {active}",
                onclick: move |_| {
                    cx.props.on_pressed.call(());
                },
                div {
                    class: "pfp"
                },
                div {
                    class: "who",
                    div {
                        class: "top-row",
                        h3 {
                            "{username}"
                        },
                        span {
                            class: "timestamp",
                            "10:00am"
                        }
                    },
                    span {
                        match *unread_count2.current() {
                            0 => rsx!("{l.chat_placeholder}"),
                            _ => rsx!("unread: {unread_count2}"),
                        }
                    }
                }
            }
        })
    }
}
