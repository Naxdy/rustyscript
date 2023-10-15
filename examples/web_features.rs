use std::time::Duration;

///
/// This example shows features requiring the 'web' feature to work
/// Stuff like setTimeout, atob/btoa, file reads and fetch are all examples
///
/// We will focus on timers here
///
use rustyscript::{json_args, Error, Module, Runtime, RuntimeOptions};

fn main() -> Result<(), Error> {
    // This module has an async function, which is not itself a problem
    // However, it uses setTimeout - the timer will never be triggered
    // unless the web feature is active.
    // See above for a longer list for web feature exclusives
    let module = Module::new(
        "test.js",
        "
        const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
        export async function test() {
            await sleep(100);
            return 2;
        }
        ",
    );

    // We add a timeout to the runtime anytime async might be used
    let mut runtime = Runtime::new(RuntimeOptions {
        timeout: Duration::from_millis(500),
        ..Default::default()
    })?;

    // The async function
    let module_handle = runtime.load_module(&module)?;
    let value: usize = runtime.call_function(&module_handle, "test", json_args!())?;
    assert_eq!(value, 2);
    Ok(())
}
