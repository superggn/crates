use clap::{Parser, Subcommand};

/// Simple program to greet a person
/// 如果要按照位置解析参数， 直接注释掉 #[arg(...)] 部分即可
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    #[command(subcommand)]
    sub: MySubCommand,
}

#[derive(Debug, Subcommand)]
enum MySubCommand {
    Option1,
    Option2,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
