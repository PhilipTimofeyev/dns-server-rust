use codecrafters_dns_server::run;

fn main() {
    if let Err(err) = run() {
        eprint!("{err}");
        std::process::exit(1);
    }
}
