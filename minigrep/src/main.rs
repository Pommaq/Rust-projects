use std::process::exit;

fn main() {
    let config = minigrep::Config::new().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        exit(1);
    }

}
