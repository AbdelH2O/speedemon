use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The link to the file to download
    #[clap(short, long)]
    link: String,

    /// The output file. If not specified, the file will be saved in the current directory
    #[clap(short, long, default_value = ".")]
    output: String,

    /// The number of threads to use for downloading
    #[clap(short = 'p', long, default_value = "4")]
    threads: u32,

    /// The number of retries to use for downloading
    #[clap(short, long, default_value = "3")]
    retries: u32,

    /// The timeout for each request
    #[clap(short, long, default_value = "10")]
    timeout: u32,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
