use colored::Colorize;

pub enum LogRequestKing {
    Tar,
    TarUrl,
    TarAndTarUrl
}

pub fn log_ok(kind: LogRequestKing, pkg_name: &str) {
    println!(
        "{} {} {} {}",
        "[OK]".bold().green(),
        {match kind {
            LogRequestKing::Tar => "[TAR]",
            LogRequestKing::TarUrl => "[TAR_URL]",
            LogRequestKing::TarAndTarUrl => "[TAR_URL+TAR]"
        }}.bold().cyan(),
        pkg_name.blue(),
        "perfect".dimmed()
    )
}

pub fn log_err(kind: LogRequestKing, pkg_name: &str, err: String) {
    println!(
        "{} {} {} {}",
        "[ERR]".bold().red(),
        {match kind {
            LogRequestKing::Tar => "[TAR]",
            LogRequestKing::TarUrl => "[TAR_URL]",
            LogRequestKing::TarAndTarUrl => "[TAR_URL+TAR]"
        }}.bold().cyan(),
        pkg_name.blue(),
        err
    )
}