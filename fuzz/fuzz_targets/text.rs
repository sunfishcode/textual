#![no_main]

#[macro_use]
extern crate libfuzzer_sys;

use io_ext::{SliceReader, WriteExt};
use io_ext_adapters::StdWriter;
use std::{
    io::{Read, Write},
    str,
};
use text_streams::{TextReader, TextWriter};
use unicode_normalization::UnicodeNormalization;

fuzz_target!(|bytes: &[u8]| {
    let mut reader = TextReader::new(SliceReader::new(bytes));
    let mut writer = TextWriter::new(StdWriter::new(Vec::<u8>::new()));

    let mut s = String::new();
    reader.read_to_string(&mut s).unwrap();

    // No BOMs.
    assert!(!s.chars().any(|c| c == '\u{feff}'));

    // No ORCs.
    assert!(!s.chars().any(|c| c == '\u{fffc}'));

    // Trailing newline.
    assert!(s.is_empty() || s.ends_with('\n'));

    // No control codes other than '\n' and '\t'.
    assert!(!s.chars().any(|c| c.is_control() && c != '\n' && c != '\t'));

    // Stream-Safe NFC.
    assert!(unicode_normalization::is_nfc_stream_safe(&s));

    // Don't start with non-starter.
    if let Some(first) = s.chars().next() {
        assert!(unicode_normalization::char::canonical_combining_class(first) == 0);
    }

    // Writing it back out to a text writer should preserve the bytes.
    writer.write_str(&s).unwrap();
    let inner = writer.close_into_inner().unwrap();
    assert_eq!(inner.get_ref(), s.as_bytes());

    // Iff a text reader normalized something, that same thing should fail
    // when written as output.
    let mut writer = TextWriter::new(StdWriter::new(Vec::<u8>::new()));
    match str::from_utf8(bytes) {
        Ok(utf8) => {
            let result = writer
                .write_all(bytes)
                .and_then(|()| writer.close_into_inner());
            if utf8.chars().svar().stream_safe().nfc().collect::<String>() == s {
                result.unwrap();
            } else {
                result.map(|_| ()).unwrap_err();
            }
        }
        Err(_) => {
            writer
                .write_all(bytes)
                .and_then(|()| writer.close_into_inner().map(|_| ()))
                .unwrap_err();
        }
    }
});