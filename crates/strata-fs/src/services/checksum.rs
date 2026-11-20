// SPDX-License-Identifier: MIT

use std::{
    fmt::Write as _,
    io::{self, Read},
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

const CHUNK: usize = 1 << 20;

/// The lowercase hexadecimal SHA-256 of the file at `path`, or `None` once
/// `cancelled` is set. Blocks while it reads, so call it off the interface thread.
///
/// # Errors
///
/// Returns the error from opening or reading the file.
pub fn sha256_file(path: &Path, cancelled: &AtomicBool) -> io::Result<Option<String>> {
    let mut file = std::fs::File::open(path)?;
    let mut context = ring::digest::Context::new(&ring::digest::SHA256);
    let mut buffer = vec![0; CHUNK];
    loop {
        if cancelled.load(Ordering::Relaxed) {
            return Ok(None);
        }
        let read = match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(read) => read,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        };
        context.update(&buffer[..read]);
    }
    let mut hex = String::with_capacity(64);
    for byte in context.finish().as_ref() {
        let _ = write!(hex, "{byte:02x}");
    }
    Ok(Some(hex))
}

#[cfg(test)]
mod tests;
