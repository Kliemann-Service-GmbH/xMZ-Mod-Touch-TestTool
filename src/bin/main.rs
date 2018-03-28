extern crate xmz_test_tool;

use xmz_test_tool::errors::TestToolError;

fn run() -> Result<(), TestToolError> {
    xmz_test_tool::gui::gtk3::launch();

    Ok(())
}


fn main() {
    println!("{} Version: {}\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"));


    if let Err(ref e) = run() {
        println!("Error: {}", e);

        ::std::process::exit(1);
    }
}
