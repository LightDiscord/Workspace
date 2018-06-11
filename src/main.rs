#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use std::io;

pub mod commands;
use commands::{ mount };

#[macro_use]
mod utils;

#[derive(StructOpt, Debug)]
#[structopt(name = "workspace")]
struct Workspace {
    #[structopt(subcommand)]
    command: Commands
}

subcommands! {
    /// Mount a remote workspace
    #[structopt(name = "mount")]
    Mount(mount::Command)
}

pub trait Executor {
    fn execute (&self) -> io::Result<()>;
}

fn main() {
    let matches = Workspace::from_args();

    matches.command.execute().unwrap();

    println!("{:?}", matches);
    println!("Hello, world!");
}
