mod components;

use dioxus::prelude::*;
use components::header::Header;

pub(crate) fn App(cx: Scope) -> Element {
    render! {
        Header {}
        div {
            "Hello, world!"
        }
    }
}
