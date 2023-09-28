struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    print!(
        "path: {0}\npattern: {1}",
        args.path.to_string_lossy(),
        args.pattern
    );
}
