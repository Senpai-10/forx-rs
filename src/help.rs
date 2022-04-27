use colored::Colorize;

pub fn help() {
    println!(
        "{} forx-rs <from> <to> [options]",
        format!(" Usage ").on_bright_green().black().bold()
    );
    println!("\t\tA command line tool for checking exchange rates between currencies, both crypto and fiat");
    println!();
    println!("{}", format!(" Options ").on_bright_green().black().bold());
    entry("--help", "-h", "print help message");
    entry("--version", "-v", "Print forx-rs version");
    entry("--verbose", "-V", "Print verbose messages");
    entry("--list", "-l", "Print list of valid currencies");
    entry(
        "--quantity",
        "-q",
        "Quantity of FROM currency. Defaults to 1",
    );
}

fn entry(name: &str, alias: &str, description: &str) {
    println!(
        "{0}, {1:15}\t{2}",
        alias.bright_yellow(),
        name.bright_yellow(),
        description.bright_black()
    )
}
