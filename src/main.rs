use anyhow::Result;
use clap::Parser;
use typemap::dot::generate_dot;
use typemap::TypeMap;

#[derive(Parser)]
#[clap(
    about = "Visualize type dependence in your Rust projects",
    version = "0.1"
)]
struct Args {
    #[clap(short, long)]
    infile: String,
    #[clap(short, long)]
    outfile: Option<String>,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let graph = TypeMap::build(&args.infile)?;
    let dot = generate_dot(graph.graph(), args.outfile.as_deref());
    if let None = args.outfile {
        println!("{dot}");
    }

    Ok(())
}
