#![allow(non_snake_case)]

mod app;

fn main() {
    // Launch the Dioxus App in a webview.
    dioxus_desktop::launch(app::App);
}
