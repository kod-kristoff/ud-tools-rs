use std::io;
use std::io::Write;

use ud_tools::{
    extract::extract_text,
    parser::{parse_incr, parse_sentences},
};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    // extract_text(&mut stdin, &mut stdout)
    for sentence in parse_incr(&mut stdin) {
        let sentence = sentence?;
        writeln!(&mut stdout, "sentence: {:?}", sentence)?;
        // for token in sentence {
        //     writeln!(&mut stdout, "token: {:?}", token)?;
        // }
    }
    Ok(())
}
