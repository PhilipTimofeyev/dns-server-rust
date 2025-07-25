use codecrafters_dns_server::run_server;

fn main() {
    if let Err(err) = run_server() {
        eprint!("{err}");
        std::process::exit(1);
    }
}
