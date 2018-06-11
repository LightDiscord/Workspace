#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use std::io;

pub mod commands;
use commands::{ mount };

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
    Mount(mount::Command)
}

impl Executor for Command {
    fn execute (&self) -> io::Result<()> {
        match *self {
            Command::Mount(_) => unimplemented!()
        }
    }
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
