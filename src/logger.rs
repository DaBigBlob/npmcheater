pub fn init(logging: bool) {
    //match simple_logger::init_with_level(log::Level::Info) {_ => ()};
    match simple_logger::SimpleLogger::new()
        .without_timestamps()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .env()
        .without_timestamps()
        .init() {
            Err(e) => if !logging {println!("ERROR INITIATING LOGGER: {}", e)},
            _ => ()
        };
}