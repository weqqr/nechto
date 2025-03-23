use knopka_ui::view::{Button, Layout, Scope, Text, View};
use nechto::runtime::Runtime;

fn main() {
    tracing_subscriber::fmt::init();

    let rt = Runtime::new();
    rt.run();
}

fn sample_ui() -> impl View {
    Scope::new(|cx| {
        let name = cx.state("knopka");
        let value = cx.state(20);

        Layout::column()
            // .modifiers((
            //     Padding::new(10.0),
            //     Background::new(Color::BLACK),
            // ))
            .views((
                Text::new(move || format!("name = {}, value = {}", name, value)),
                Layout::row().views((
                    Text::new("Name:"),
                    Text::new(move || format!("{}", name)),
                    //TextEdit::single_line(name),
                )),
                Layout::row().views((
                    Text::new("Value:"),
                    Button::new("Increase").on_click(move || {
                        // value += 1;
                    }),
                    Button::new("Decrease").on_click(move || {
                        // value -= 1;
                    }),
                )),
            ))
    })
}
