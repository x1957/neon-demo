#[macro_use]
extern crate neon;

use neon::prelude::*;
use num_cpus;

fn thread_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

fn thread_count_cb(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let prefix = cx.argument::<JsString>(0)?.value();
    let f = cx.argument::<JsFunction>(1)?; // function(result) {console.log(result);}
    let result = format!("{}:{}", prefix, num_cpus::get());
    let args = vec![cx.string(result)];
    let null = cx.null();
    f.call(&mut cx, null, args)?
        .downcast::<JsUndefined>()
        .or_throw(&mut cx)
}

register_module!(mut m, {
    m.export_function("threadCount", thread_count)?;
    m.export_function("threadCountCb", thread_count_cb)?;
    Ok(())
});
