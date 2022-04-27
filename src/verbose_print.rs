use colored::Colorize;

pub fn verbose_print(msg: String, verbose: bool) -> () {
    if verbose {
        println!("{} {}", "[INFO]".bright_green(), msg);
    }
}
