use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Translates single files through the use of a local llm"
)]
pub struct Args {
    #[arg(short, long, help = "Input file")]
    pub input: String,

    #[arg(
        short,
        long,
        help = "Output file",
        default_value_t = String::from("out")
    )]
    pub out: String,

    #[arg(
        short,
        long,
        help = "Target language",
        default_value_t = String::from("en")
    )]
    pub target: String,

    #[arg(short, long, help = "llm model name")]
    pub model: String,

    #[arg(short, long, help = "some context that should be added")]
    pub context: String,
}
