pub struct Cli {
    pub help: bool,     // Print help message
    pub version: bool,  // Print forx-rs version
    pub verbose: bool,  // Print verbose messages
    pub list: bool,     // Print list of valid currencies
    pub quantity: bool, // Quantity of FROM currency. Defaults to 1.
    pub from: String,
    pub to: String,
}

impl Cli {
    pub fn new() -> Cli {
        let mut cli = Cli {
            from: String::new(),
            to: String::new(),

            help: false,
            version: false,
            verbose: false,
            list: false,
            quantity: false,
        };

        let args: Vec<String> = std::env::args().skip(1).collect();

        for arg in args.iter() {
            if !arg.starts_with("-") {
                if cli.from.is_empty() {
                    cli.from = arg.to_string()
                } else if cli.to.is_empty() {
                    cli.to = arg.to_string()
                }
            } else if arg == "--help" || arg == "-h" {
                cli.help = true;
            } else if arg == "--version" || arg == "-v" {
                cli.version = true;
            } else if arg == "--verbose" || arg == "-V" {
                cli.verbose = true;
            } else if arg == "--list" || arg == "-l" {
                cli.list = true;
            } else if arg == "--quantity" || arg == "-q" {
                cli.quantity = true;
            }
        }

        cli
    }
}
