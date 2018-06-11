use super::Executor;
use std::io;

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Create a default.nix used by nix-shell
    #[structopt(short = "s", long = "shell")]
    shell: bool,

    /// The repository to clone
    repository: String
}

impl Executor for Command {
    fn execute (&self) -> io::Result<()> {
        unimplemented!()
    }
}
