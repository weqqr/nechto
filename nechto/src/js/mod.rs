mod console;

use std::path::Path;
use std::sync::Arc;

use rquickjs::module::Declared;
use rquickjs::{CatchResultExt, Function, Module, Value};

use crate::js::console::console_object;
use crate::vfs::VirtualFs;

pub struct Context {
    runtime: rquickjs::Runtime,
    ctx: rquickjs::Context,
}

impl Context {
    pub fn new(vfs: Arc<VirtualFs>) -> Self {
        let runtime = rquickjs::Runtime::new().unwrap();
        let ctx = rquickjs::Context::full(&runtime).unwrap();

        runtime.set_loader(
            VfsResolver {
                root: "$build/script".to_string(),
                vfs: Arc::clone(&vfs),
            },
            VfsLoader { vfs },
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

pub struct VfsResolver {
    root: String,
    vfs: Arc<VirtualFs>,
}

impl rquickjs::loader::Resolver for VfsResolver {
    fn resolve(
        &mut self,
        _ctx: &rquickjs::Ctx,
        base: &str,
        name: &str,
    ) -> rquickjs::Result<String> {
        let path = Path::new(&self.root).join(name).with_extension("js");
        let path = path.to_str().unwrap();

        let exists = self
            .vfs
            .exists(path)
            .map_err(|_| rquickjs::Error::new_resolving(base, name))?;

        if exists {
            Ok(path.to_string())
        } else {
            Err(rquickjs::Error::new_resolving(base, name))
        }
    }
}

pub struct VfsLoader {
    vfs: Arc<VirtualFs>,
}

impl rquickjs::loader::Loader for VfsLoader {
    fn load<'js>(
        &mut self,
        ctx: &rquickjs::Ctx<'js>,
        name: &str,
    ) -> rquickjs::Result<Module<'js, Declared>> {
        let source: Vec<_> = self
            .vfs
            .read(name)
            .map_err(|_| rquickjs::Error::new_loading(name))?;

        Module::declare(ctx.clone(), name, source)
    }
}
