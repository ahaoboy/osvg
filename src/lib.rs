use anyhow::Ok;
use rquickjs::{
    loader::{BuiltinResolver, ModuleLoader},
    module::ModuleDef,
    Context, Ctx, Function, Module, Object, Runtime, Value,
};
#[cfg(target_os = "windows")]
const EOL: &str = "\r\n";

#[cfg(not(target_os = "windows"))]
const EOL: &str = "\n";

pub struct OsModule;

impl ModuleDef for OsModule {
    fn declare(declare: &rquickjs::module::Declarations) -> rquickjs::Result<()> {
        declare.declare("EOL")?;
        declare.declare("default")?;
        rquickjs::Result::Ok(())
    }

    fn evaluate<'js>(
        ctx: &Ctx<'js>,
        exports: &rquickjs::module::Exports<'js>,
    ) -> rquickjs::Result<()> {
        let os = Object::new(ctx.clone())?;
        os.set("EOL", EOL)?;
        exports.export("EOL", EOL)?;
        exports.export("default", os)?;
        rquickjs::Result::Ok(())
    }
}

pub fn osvg(svg: &str) -> anyhow::Result<String> {
    let runtime = Runtime::new()?;
    let context = Context::full(&runtime)?;
    let loader = (ModuleLoader::default().with_module("os", OsModule),);
    let resolver = (BuiltinResolver::default().with_module("os"),);
    runtime.set_loader(resolver, loader);

    let s = context.with(|ctx| {
        let global = ctx.globals();
        let name = "osvg.js";
        let code = include_str!("../dist/osvg.js");
        Module::evaluate(ctx.clone(), name, code)
            .unwrap()
            .finish::<Value>()?;

        let optimize: Function = global.get("optimize")?;
        // TODO: add config
        // let config = Object::new(ctx)?;
        // config.set("multipass", true)?;
        // let ret: Object = optimize.call((svg, config))?;
        let ret: Object = optimize.call((svg,))?;
        let data: String = ret.get("data")?;
        Ok(data)
    })?;

    Ok(s)
}
