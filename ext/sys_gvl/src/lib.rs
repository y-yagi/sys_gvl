use magnus::{function, prelude::*, Error, Ruby, class::object, method};
use lucchetto::without_gvl;

fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

fn slow_func(input: String) -> String {
    std::thread::sleep(std::time::Duration::from_secs(2));
    format!("Hello, {input}!")
}

#[without_gvl]
fn not_lock_slow_func(input: String) -> String {
    std::thread::sleep(std::time::Duration::from_secs(2));
    format!("Hello, {input}!")
}


#[magnus::wrap(class = "SysGvlCls")]
struct SysGvlCls {
}

impl SysGvlCls {
    fn new() -> Self {
        Self {}
    }

    fn instance_slow_func(&self, input: String) -> String {
        std::thread::sleep(std::time::Duration::from_secs(2));
        format!("Hello, {input}!")
    }

    fn instance_unlock_slow_func(&self, input: String) -> String {
        std::thread::sleep(std::time::Duration::from_secs(2));
        format!("Hello, {input}!")
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("SysGvl")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    module.define_singleton_method("slow_func", function!(slow_func, 1))?;
    module.define_singleton_method("not_lock_slow_func", function!(not_lock_slow_func, 1))?;

    let class = ruby.define_class("SysGvlCls", object())?;
    class.define_singleton_method("new", function!(SysGvlCls::new, 0))?;
    class.define_method("slow_func", method!(SysGvlCls::instance_slow_func, 1))?;
    class.define_method("not_lock_slow_func", method!(SysGvlCls::instance_unlock_slow_func, 1))?;

    Ok(())
}
