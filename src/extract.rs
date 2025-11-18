use std::{io, sync::LazyLock};

use regex::Regex;

pub fn extract_text(in_: &mut dyn io::BufRead, out: &mut dyn io::Write) -> io::Result<()> {
    static NODE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(\d+)\t(.*)").expect("valid regex"));
    let mut line = String::new();
    loop {
        let rd = in_.read_line(&mut line)?;
        if rd == 0 {
            break;
        }
        if let Some(caps) = NODE.captures(&line) {
            writeln!(out, "match")?;
            write!(out, "{}", &caps[1])?;
        }
        write!(out, "{}", line)?;
        line.clear();
    }
    Ok(())
}
