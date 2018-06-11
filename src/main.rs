#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "workspace")]
struct Workspace {
    #[structopt(subcommand)]
    command: Command
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Mount a remote workspace
    #[structopt(name = "mount")]
    Mount
}

fn main() {
    let matches = Workspace::from_args();

    println!("{:?}", matches);
    println!("Hello, world!");
}
