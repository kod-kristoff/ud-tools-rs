use std::{
    io::{self, Lines},
    mem,
};

pub fn parse_incr<R: io::BufRead>(
    in_file: R,
) -> impl Iterator<Item = Result<TokenList, io::Error>> {
    SentenceGenerator {
        sentences: parse_sentences(in_file),
        global_columns: Vec::new(),
    }
}

#[derive(Debug)]
pub struct TokenList {}

struct SentenceGenerator<R: io::BufRead> {
    sentences: SentenceIter<R>,
    global_columns: Vec<String>,
}

impl<R: io::BufRead> Iterator for SentenceGenerator<R> {
    type Item = Result<TokenList, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(sentence) = self.sentences.next() {
                dbg!(&sentence);
                let sentence = match sentence {
                    Ok(x) => x,
                    Err(err) => return Some(Err(err)),
                };
                let mut curr_metadata = Vec::new();
                let mut curr_sentence = Vec::new();
                for line in sentence {
                    //     let line = match line {
                    //         Ok(x) => x,
                    //         Err(err) => return Some(Err(err)),
                    //     };
                    if line.starts_with('#') {
                        if line.starts_with("# global.columns = ") {
                            if let Some(gc) = line.splitn(1, '=').skip(1).next() {
                                self.global_columns =
                                    gc.trim().split(' ').map(ToString::to_string).collect();
                            }
                        }
                        curr_metadata.push(line);
                    } else {
                        curr_sentence.push(line);
                    }
                }
                return parse_token_and_metadata(curr_metadata, curr_sentence);
            } else {
                break;
            }
        }
        None
    }
}

pub fn parse_sentences<R: io::BufRead>(in_file: R) -> SentenceIter<R> {
    SentenceIter {
        lines: in_file.lines(),
        buf: Vec::new(),
    }
}

fn parse_token_and_metadata()

struct SentenceIter<R: io::BufRead> {
    lines: Lines<R>,
    buf: Vec<String>,
}

impl<R: io::BufRead> Iterator for SentenceIter<R> {
    type Item = Result<Vec<String>, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(line) = self.lines.next() {
                let line = match line {
                    Ok(x) => x,
                    Err(err) => return Some(Err(err)),
                };
                if line.trim().is_empty() {
                    if self.buf.is_empty() {
                        continue;
                    }
                    return Some(Ok(mem::take(&mut self.buf)));
                } else {
                    self.buf.push(line);
                }
            } else {
                break;
            }
        }
        if !self.buf.is_empty() {
            return Some(Ok(mem::take(&mut self.buf)));
        }
        None
    }
}
