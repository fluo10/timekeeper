#[derive(Debug, StructOpt)]
struct Cli {
    file: String,
    #[structopt(short = "n")]
    num: usize,
}