use cargo_specbin::*;

use clap::Parser;

fn main() -> Result<()> {
    let app = App::default();

    let args = Args::parse();
    app.run(args)
}
