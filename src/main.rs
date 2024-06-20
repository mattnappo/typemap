use anyhow::Result;
use clap::Parser;
use typemap::TypeMap;

#[derive(Parser)]
#[clap(
    about = "Visualize type dependence in your Rust projects",
    version = "0.1",
    author = "Matt Nappo"
)]
struct Args {
    #[clap(short, long)]
    file: String,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    TypeMap::build(&args.file)?;

    Ok(())
}
