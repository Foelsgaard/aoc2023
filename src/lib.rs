#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

use std::io::{BufReader, Read};
use std::{env, fs};

pub fn read_input(buf: &mut [u8]) -> &[u8] {
    let path = env::args().nth(1).unwrap();
    let file = fs::File::open(path).unwrap();
    let mut rdr = BufReader::new(file);
    let n = rdr.read(buf).unwrap();
    &buf[..n]
}

pub struct Parser<'m> {
    ix: usize,
    buf: &'m [u8],
}

impl<'m> Parser<'m> {
    pub fn new(buf: &'m [u8]) -> Parser<'m> {
        Parser { ix: 0, buf }
    }
    pub fn parse_digit(&mut self) -> Option<u8> {
        loop {
            let byte = *self.buf.get(self.ix)?;
            if b'0' <= byte && byte <= b'9' {
                self.ix += 1;
                return Some(byte);
            } else if is_whitespace(byte) {
                self.ix += 1;
            } else {
                return None;
            }
        }
    }

    pub fn peek<T>(&mut self, f: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        let start = self.ix;
        let result = f(self);
        self.ix = start;
        result
    }

    pub fn done(&self) -> bool {
        self.ix >= self.buf.len()
    }

    pub fn index(&self) -> usize {
        self.ix
    }

    pub fn next(&mut self) -> Option<u8> {
        let byte = *self.buf.get(self.ix)?;
        self.ix += 1;
        Some(byte)
    }

    pub fn parse_usize(&mut self) -> Option<usize> {
        let mut end = self.ix;
        let mut is_parsing = false;
        let mut number = 0;
        loop {
            let byte = *self.buf.get(end)?;
            if b'0' <= byte && byte <= b'9' {
                number *= 10;
                number += (byte - b'0') as usize;
                is_parsing = true;
            } else if is_parsing {
                self.ix = end;
                return Some(number);
            } else if !is_whitespace(byte) {
                return None;
            }
            end += 1;
        }
    }

    pub fn parse_usize_n<const N: usize>(&mut self) -> Option<[usize; N]> {
        let mut out = [0; N];
        for i in 0..N {
            out[i] = self.parse_usize()?;
        }

        Some(out)
    }

    pub fn parse_exact(&mut self, tgt: &[u8]) -> Option<&'m [u8]> {
        let start = self.ix;
        let mut i = 0;

        loop {
            let byte = *self.buf.get(self.ix + i)?;
            if i == tgt.len() {
                self.ix += i;
                return Some(&self.buf[start..self.ix]);
            } else if byte != tgt[i] {
                return None;
            }

            i += 1;
        }
    }

    pub fn skip_matching(&mut self, tgt: u8) -> Option<&'m [u8]> {
        let start = self.ix;
        let mut end = self.ix;

        loop {
            let byte = *self.buf.get(end)?;
            if byte == tgt {
                end += 1;
            } else {
                self.ix = end;
                return Some(&self.buf[start..end]);
            }
        }
    }

    pub fn skip_whitespace(&mut self) -> Option<&'m [u8]> {
        let start = self.ix;
        let mut end = self.ix;
        loop {
            let byte = *self.buf.get(end)?;
            if !is_whitespace(byte) {
                self.ix = end;
                return Some(&self.buf[start..end]);
            }
            end += 1;
        }
    }

    pub fn skip_line(&mut self) -> Option<&'m [u8]> {
        let start = self.ix;
        let mut end = self.ix;

        loop {
            let byte = *self.buf.get(end)?;
            end += 1;
            if byte == b'\n' {
                self.ix = end;
                return Some(&self.buf[start..end]);
            }
        }
    }
}

fn is_whitespace(byte: u8) -> bool {
    byte == b' ' || byte == b'\n'
}
