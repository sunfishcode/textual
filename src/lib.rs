//! Basic Text strings and I/O streams
//!
//! This crate provides several utilities for working with [Basic Text].
//!
//!  - [`TextString`] and [`TextStr`] are similar to the standard library's [`String`]
//!    and [`str`], but use the Basic Text string format, along with a `text!("...")`
//!    [macro] for Basic Text string literals.
//!
//!  - [`TextReader`] and [`TextWriter`] are input and output streams which use the
//!    Basic Text stream format. On input, content is converted in a way which is
//!    lossy with respect to the original bytestream. Output uses the "strict"
//!    conversion method, in which invalid content is diagnosed with errors.
//!
//!  - [`TextDuplexer`] is a [`Duplex`] for reading and writing on an interactive
//!    stream using Basic Text.
//!
//! [Basic Text format]: https://github.com/sunfishcode/basic-text/blob/main/docs/BasicText.md#basic-text

#![deny(missing_docs)]
#![cfg_attr(can_vector, feature(can_vector))]
#![cfg_attr(write_all_vectored, feature(write_all_vectored))]
#![cfg_attr(try_reserve, feature(try_reserve))]
#![cfg_attr(pattern, feature(pattern))]
#![cfg_attr(shrink_to, feature(shrink_to))]

mod copy;
mod read_text;
mod text_duplexer;
mod text_input;
mod text_output;
mod text_reader;
mod text_string;
mod text_substring;
mod text_writer;
mod write_text;

pub use basic_text_internals::unicode::NORMALIZATION_BUFFER_SIZE;
pub use basic_text_literals::{text, text_substr};
pub use copy::{copy_text, copy_text_using_status};
pub use read_text::{default_read_exact_text, ReadText, ReadTextLayered};
pub use text_duplexer::TextDuplexer;
pub use text_reader::TextReader;
pub use text_string::{default_read_to_text_string, FromTextError, TextError, TextStr, TextString};
pub use text_substring::{default_read_to_text_substring, TextSubstr, TextSubstring};
pub use text_writer::TextWriter;
pub use write_text::{default_write_text, WriteText};
