use clap::Parser;

#[derive(Parser)]
struct Cli {
  currency: String,
  amount: f32,
}

fn main() {
  let args = Cli::parse();
  if args.currency == "eur" {
    println!("{:.2}", args.amount * 7.5345);
  } else if args.currency == "hrk" {
    println!("{:.2}", args.amount / 7.5345);
  } else {
    println!("Invalid currency. Supported currencies are <eur> and <hrk>.");
  }
}