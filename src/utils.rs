macro_rules! subcommands {
    (
        $(
            $(#[$meta:meta])*
            $name:ident($struct:path)
        )*
    ) => {
        #[derive(StructOpt, Debug)]
        pub enum Commands {
            $(
                $(#[$meta])*
                $name($struct)
            ),*
        }

        impl Executor for Commands {
            fn execute (&self) -> io::Result<()> {
                match *self {
                    $(Commands::$name(ref command) => command.execute()),*
                }
            }
        }
    };
}
