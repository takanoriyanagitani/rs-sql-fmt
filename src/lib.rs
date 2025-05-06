use std::io;

use std::io::Write;

use std::io::Read;

use sqlformat::FormatOptions;
use sqlformat::Indent;
use sqlformat::QueryParams;

pub const FORMAT_DEFAULT: FormatOptions = FormatOptions {
    indent: Indent::Spaces(2),
    uppercase: Some(true),
    lines_between_queries: 1,
    ignore_case_convert: None,
};

pub fn sql2formatted_with_opt(query: &str, opt: &FormatOptions) -> String {
    let params: QueryParams = QueryParams::default();
    sqlformat::format(query, &params, opt)
}

pub fn sql2formatted(query: &str) -> String {
    sql2formatted_with_opt(query, &FORMAT_DEFAULT)
}

pub fn sql2formatted2writer<W>(mut w: W) -> impl FnMut(&str) -> Result<(), io::Error>
where
    W: Write,
{
    move |query: &str| {
        let formatted: String = sql2formatted(query);
        writeln!(&mut w, "{formatted}")?;
        w.flush()
    }
}

pub fn sql2formatted2stdout() -> impl FnMut(&str) -> Result<(), io::Error> {
    let o = io::stdout();
    let l = o.lock();
    sql2formatted2writer(l)
}

pub fn reader2query_limited<R>(limit: u64) -> impl Fn(R) -> Result<String, io::Error>
where
    R: Read,
{
    move |rdr: R| {
        let mut limited = rdr.take(limit);
        let mut out: String = String::new();
        limited.read_to_string(&mut out)?;
        Ok(out)
    }
}

pub const SQL_LEN_LIMIT_DEFAULT: u64 = 1048576;

pub fn reader2query_default<R>(rdr: R) -> Result<String, io::Error>
where
    R: Read,
{
    reader2query_limited(SQL_LEN_LIMIT_DEFAULT)(rdr)
}

pub fn stdin2query_default() -> Result<String, io::Error> {
    let i = io::stdin();
    let l = i.lock();
    reader2query_default(l)
}

pub fn stdin2sql2formatted2stdout() -> Result<(), io::Error> {
    let sql: String = stdin2query_default()?;
    sql2formatted2stdout()(&sql)
}
