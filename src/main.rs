extern crate xmz_mod_touch_test_tool;

use xmz_mod_touch_test_tool::errors::*;

fn run() -> Result<()> {
    xmz_mod_touch_test_tool::gui::gtk3::launch();

    Ok(())
}


fn main() {
    println!("{} Version: {}\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"));


    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
