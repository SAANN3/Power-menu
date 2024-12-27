use dioxus::prelude::*;
use dioxus::{document::Title, prelude::{component, rsx, Element}};

use crate::HoverProps;

#[component]
pub fn WindowItem(text: String, image: String, position: i64, selected: Signal<Option<i64>>,  onhover: EventHandler<HoverProps>) -> Element {
    let text = use_signal(|| text);
    let mut transform = use_signal(|| "".to_string());
    let mut opacity = use_signal(|| 1.0);
    let mut z = use_signal(|| 0);
    let mut call_transform = move || {
        if selected.read().is_none() {
            *selected.write() = Some(position);
        }
        // used to test
        // else {
        //    *selected.write() = None;
        //}
        
    };

    use_effect(move || {
        if selected.read().is_some() {
            *transform.write() = format!("translateX(calc({position}*(100% + 40px)))");
            if position != selected.read().unwrap() {
                *opacity.write() = 0.0;
                *z.write() = -9999;
            }
        } else {
            *transform.write() = "".to_string();
            *opacity.write() = 1.0;
            *z.write() = 0;
        }
    });

    rsx! {
        div {
            onmouseenter: move |_| onhover.call(HoverProps{inside: true, text: text()}),
            onmouseleave: move |_| onhover.call(HoverProps{inside: false,text: text()}),
            onclick: move |_| call_transform(),
            class: "vbox window-item",
            transform: transform,
            opacity: opacity,
            z_index: z,
            img { 
                src: image, class: "image-with-text"
            },
            div {font_size: "28px",
                "{text}"
            }
        }
    }
}