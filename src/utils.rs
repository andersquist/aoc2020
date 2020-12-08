use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

pub fn parse_newline_sep<T>(path: &Path) -> std::io::Result<impl '_ + Iterator<Item = T>>
where
  T: FromStr,
  <T as FromStr>::Err: std::fmt::Display,
{
  let file = File::open(path)?;
  let mut reader = io::BufReader::new(file);
  let mut buf = String::new();
  let mut line: usize = 0;

  fn is_new_field(buf: &str) -> bool {
    let patterns = ["\n\n", "\n\r\n"];
    patterns.iter().any(|pat| {
      buf
        .as_bytes()
        .iter()
        .rev()
        .zip(pat.as_bytes().iter())
        .all(|(b, p)| b == p)
    })
  }

  Ok(
    std::iter::from_fn(move || {
      buf.clear();
      while buf.is_empty() || !is_new_field(&buf) {
        line += 1;
        if reader.read_line(&mut buf).ok()? == 0 {
          break;
        }
      }
      if buf.is_empty() {
        None
      } else {
        match T::from_str(&buf) {
          Ok(t) => Some(t),
          Err(e) => {
            eprintln!(
              "{}:{}: {}",
              path
                .file_name()
                .expect("File::open() didn't early return before now; qed")
                .to_string_lossy(),
              line - 1,
              e
            );
            None
          }
        }
      }
    })
    .fuse(),
  )
}

pub fn parse_singleline_sep<T>(path: &Path) -> std::io::Result<impl '_ + Iterator<Item = T>>
where
  T: FromStr,
  <T as FromStr>::Err: std::fmt::Display,
{
  let file = File::open(path)?;
  let mut reader = io::BufReader::new(file);
  let mut buf = String::new();
  let mut line: usize = 0;

  fn is_new_field(buf: &str) -> bool {
    let patterns = ["\n", "\r\n"];
    patterns.iter().any(|pat| {
      buf
        .as_bytes()
        .iter()
        .rev()
        .zip(pat.as_bytes().iter())
        .all(|(b, p)| b == p)
    })
  }

  Ok(
    std::iter::from_fn(move || {
      buf.clear();
      while buf.is_empty() || !is_new_field(&buf) {
        line += 1;
        if reader.read_line(&mut buf).ok()? == 0 {
          break;
        }
      }
      if buf.is_empty() {
        None
      } else {
        match T::from_str(&buf) {
          Ok(t) => Some(t),
          Err(e) => {
            eprintln!(
              "{}:{}: {}",
              path
                .file_name()
                .expect("File::open() didn't early return before now; qed")
                .to_string_lossy(),
              line - 1,
              e
            );
            None
          }
        }
      }
    })
    .fuse(),
  )
}

pub fn parse<T>(path: &Path) -> std::io::Result<impl '_ + Iterator<Item = T>>
where
  T: FromStr,
  <T as FromStr>::Err: std::fmt::Display,
{
  let file = File::open(path)?;
  let mut reader = BufReader::new(file);
  let mut buf = String::new();
  let mut line: usize = 0;

  let iter = std::iter::from_fn(move || {
    buf.clear();
    reader.read_line(&mut buf).ok().and_then(|_| {
      line += 1;
      if buf.is_empty() {
        None
      } else {
        match T::from_str(&buf.trim()) {
          Ok(t) => Some(t),
          Err(e) => {
            eprintln!(
              "{}:{}: {} for {:?}",
              path
                .file_name()
                .expect("File::open() failed.")
                .to_string_lossy(),
              line,
              e,
              buf,
            );
            None
          }
        }
      }
    })
  });
  Ok(iter)
}
