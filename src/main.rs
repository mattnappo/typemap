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
    /// Rust file to analyze.
    #[clap(short, long)]
    infile: String,
    /// PDF file to output to. If none, will print dot to stdout.
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
