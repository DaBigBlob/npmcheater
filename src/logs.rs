use colored::Colorize;

pub enum LogRequestKing {
    Tar,
    TarUrl,
    TarAndTarUrl
}

pub fn log_ok(kind: LogRequestKing, pkg_name: &str, count: usize) {
    println!(
        "{} {} {} {}",
        "[OK]".bold().green(),
        {match kind {
            LogRequestKing::Tar => "[TAR]",
            LogRequestKing::TarUrl => "[TAR_URL]",
            LogRequestKing::TarAndTarUrl => "[TAR_URL+TAR]"
        }}.bold().cyan(),
        pkg_name.blue(),
        count.to_string().dimmed()
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

pub fn log_sleep(ms: u64) {
    println!(
        "{} {}",
        "[SLEEPING]".bold().yellow(),
        (ms.to_string()+"ms").dimmed()
    )
}