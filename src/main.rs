use std::process::ExitCode;

use std::io;

fn sub() -> Result<(), io::Error> {
    rs_sql_fmt::stdin2sql2formatted2stdout()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
