use clap::Parser;

#[derive(Parser)]
#[command(author = "s-kybound")] 
#[command(version = "0.0.1")]
#[command(about = "Cody language compiler", long_about = None)]
struct Args {
    #[arg(short = 'i', long = "input")]
    input_file: String,

    #[arg(default_value = "a.out")]
    #[arg(short = 'o', long = "output")]
    output_file: String,
}

pub fn read_args() -> (String, String) {
    let args = Args::parse();
    (args.input_file, args.output_file)
}

