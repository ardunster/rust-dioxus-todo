use dioxus::prelude::*;

pub(crate) fn Header(cx: Scope) -> Element {
    render! {
        h1 {
            "TODO List"
        }
    }
}
