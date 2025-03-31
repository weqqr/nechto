use nechto::runtime::{App, Runtime};

pub struct Editor {}

impl App for Editor {}

fn main() {
    tracing_subscriber::fmt::init();

    let rt = Runtime::new(Editor {});
    rt.run();
}
