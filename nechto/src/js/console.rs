use rquickjs::function::IntoJsFunc;
use rquickjs::prelude::Rest;
use rquickjs::{Ctx, Function, Object, Value};
use tracing::info;

pub fn console_object(ctx: Ctx) -> Object {
    let object = Object::new(ctx.clone()).unwrap();

    set_function(ctx.clone(), &object, "log", log);

    object
}

fn set_function<'a, P, F>(ctx: Ctx<'a>, object: &Object<'a>, name: &str, func: F)
where
    F: IntoJsFunc<'a, P> + 'a,
{
    object
        .set(
            name,
            Function::new(ctx, func).unwrap().with_name(name).unwrap(),
        )
        .unwrap();
}

fn log(rest: Rest<Value>) {
    for obj in rest.0 {
        info!("{:?}", obj);
    }
}
