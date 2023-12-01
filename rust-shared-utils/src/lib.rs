pub fn read_puzzle() -> std::io::Result<String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "please provide an input file",
        ));
    }

    std::fs::read_to_string(&args[1])
}
