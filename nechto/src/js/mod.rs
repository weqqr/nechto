mod console;

use rquickjs::loader::{BuiltinResolver, FileResolver, ScriptLoader};
use rquickjs::{CatchResultExt, Function, Module, Value};

use crate::js::console::console_object;

pub struct Context {
    runtime: rquickjs::Runtime,
    ctx: rquickjs::Context,
}

impl Context {
    pub fn new() -> Self {
        let runtime = rquickjs::Runtime::new().unwrap();
        let ctx = rquickjs::Context::full(&runtime).unwrap();

        runtime.set_loader(
            (
                BuiltinResolver::default(),
                FileResolver::default().with_path("./build/script"),
            ),
            (ScriptLoader::default(),),
        );

        ctx.with(|ctx| {
            let globals = ctx.globals();

            globals.set("console", console_object(ctx.clone())).unwrap();

            globals
                .set(
                    "print",
                    Function::new(ctx.clone(), |value: Value| {
                        println!("{:?}", value);
                    })
                    .unwrap()
                    .with_name("print")
                    .unwrap(),
                )
                .unwrap();

            Module::evaluate(
                ctx.clone(),
                "main",
                r#"
                    import 'init';
                "#,
            )
            .catch(&ctx)
            .unwrap()
            .finish::<()>()
            .catch(&ctx)
            .unwrap();
        });

        Self { runtime, ctx }
    }
}
