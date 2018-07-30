#![feature(use_extern_macros)]


use relm_attributes::widget;

struct Model {
    // …
}

#[derive(Msg)]
enum Msg {
    // …
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(event: Msg) -> RetType {
        unimplemented!();
    }
}
