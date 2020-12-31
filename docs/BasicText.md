# Basic Text

The *Basic Text* format is built on top of the [Unicode] format. It is intended
for general-purpose use most anywhere the informal notion of "plain text" is
intended.

Basic text does permit homoglyphs and other visual ambiguities; see
[Restricted Text] for an alternative which provides some mitigations.

## Definitions

A string is in Basic Text form iff:
 - it is a [Unicode] string in [Stream-Safe] [NFC] form, and
 - it starts with a [starter] other than U+200D (ZWJ), and
 - it does not contain any of the sequences listed in the [Tables].

A substring is in Basic Text form iff:
 - it is itself a string in Basic Text form, and
 - it the boundaries of the substring within the parent string are
   [Grapheme Cluster Boundaries].

A stream is in Basic Text form iff:
 - it consists entirely of a string in Basic Text form, and
 - it is empty or ends with U+A.

A buffered stream is in Basic Text form iff:
 - the stream is in Basic Text form, and
 - a successful buffer flush produces output which is a substring in Basic Text
   form.

[Tables]: #tables
[starter]: https://unicode.org/reports/tr15/#Description_Norm
[non-starter]: https://unicode.org/reports/tr15/#Description_Norm
[Grapheme Cluster Boundaries]: http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries

## Tables

### Pre-NFC Table

| Sequence            | aka  | Replacement       | Error                                     |
| ------------------- | ---- | ----------------- | ----------------------------------------- |
| U+2126              | `Ω`  | U+3A9             | "Use U+03A9 instead of U+2126"            |
| U+212A              | `K`  | U+4B              | "Use U+004B instead of U+212A"            |
| U+212B              | `Å`  | U+C5              | "Use U+00C5 instead of U+212B"            |
| U+2329              | `〈` | U+3008            | "Unicode deprecated U+2329"               |
| U+232A              | `〉` | U+3009            | "Unicode deprecated U+232A"               |
| [CJK Compatibility Ideographs] | | [Standardized Variant] | "Use Standardized Variants instead of CJK Compatibility Ideographs" |

### Main Table

| Sequence            | aka  | Replacement       | Error                                     |
| ------------------- | ---- | ----------------- | ----------------------------------------- |
| U+D U+A             | CRLF | U+A               | "Use U+A to terminate a line"             |
| U+D                 | CR   | U+A               | "Use U+A to terminate a line"             |
| U+C                 | FF   | U+20              | "Control character not valid in text"     |
| U+1B U+5B \[U+20–U+3F\]\* U+6D                 | SGR | | "Color escape sequences are not enabled" |
| U+1B+ U+5B \[U+20–U+3F\]\* \[U+40–U+7E\]?      | CSI | | "Unrecognized escape sequence"           |
| U+1B+ U+5D \[\^U+7,U+18,U+1B\]\* \[U+7,U+18\]? | OSC | | "Unrecognized escape sequence"           |
| U+1B+ \[U+40–U+7E\]?                           | ESC | | "Unrecognized escape sequence"           |
| U+1B+ U+5B U+5B \[U+–U+7F\]?                   |     | | "Unrecognized escape sequence"           |
| \[U+0–U+8,U+B,U+E–U+1F\] | C0  | U+FFFD        | "Control character not valid in text"     |
| U+7F                | DEL  | U+FFFD            | "Control character not valid in text"     |
| U+85                | NEL  | U+20              | "Control character not valid in text"     |
| \[U+80–U+84,U+86–U+9F\] | C1 | U+FFFD          | "Control character not valid in text"     |
| U+149               | `ʼn` | U+2BC U+6E        | "Use U+2BC U+6E instead of U+149"         |
| U+673               | `ا ٟ` | U+627 U+65F       | "Use U+627 U+65F instead of U+673"        |
| U+F77               | `◌ྲ◌ཱྀ` | U+FB2 U+F81       | "Use U+FB2 U+F81 instead of U+F77         |
| U+F79               | `◌ླ◌ཱྀ` | U+FB3 U+F81       | "Use U+FB3 U+F81 instead of U+F79         |
| U+17A3              | `អ`  | U+17A2            | "Use U+17A2 instead of U+17A3"            |
| U+17A4              | `អា` | U+17A2 U+17B6     | "Use U+17A2 U+17B6 instead of U+17A4"     |
| U+17B4              |      | U+FFFD            | "Unicode discourages use of U+17B4"       |
| U+17B5              |      | U+FFFD            | "Unicode discourages use of U+17B5"       |
| U+17D8              |      | U+FFFD            | "Unicode discourages use of U+17D8"       |
| U+FEFF              | BOM  | U+2060            | "U+FEFF is not necessary in text"         |
| U+FFFC              | ORC  | U+FFFD            | "U+FFFC depends on out-of-band information" |
| \[U+FFF9–U+FFFB\]   |      | U+FFFD            | "Interlinear Annotations depend on out-of-band information" |
| [Noncharacters]                | | U+FFFD      | "Noncharacters depend on private agreements" |
| [Deprecated Format Characters] | | U+FFFD      | "Deprecated Format Characters are deprecated" |
| [Private-Use Characters]       | | U+FFFD      | "Private-use characters depend on private agreements" |
| [Tag Characters]               | | U+FFFD      | "Tag Characters do not belong to textual content" |

## Conversion

### String Conversion, Lossy

To convert a [Unicode] string into a Basic Text string in a manner that always
succeeds but potentially loses information:
 - If the input starts with a [non-starter] or U+200D (ZWJ), replace it with
   U+FFFD.
 - Perform the Replacement actions from the [Pre-NFC Table].
 - Perform the [Stream-Safe Text Process (UAX15-D4)].
 - Perform `toNFC` with the [Normalization Process for Stabilized Strings].
 - Perform the Replacement actions from the [Main Table].

### String Conversion, Strict

To convert a [Unicode] string into a Basic Text string in a manner that never
loses information but may fail:
 - If the input starts with a [non-starter] or U+200D (ZWJ), error with
   "Basic Text string must begin with a starter other than ZWJ"
 - Perform the Error actions from the [Pre-NFC Table].
 - Perform the [Stream-Safe Text Process (UAX15-D4)].
 - Perform `toNFC` with the [Normalization Process for Stabilized Strings].
 - Perform the Error actions from the [Main Table].

### Stream Conversion, Lossy

To convert a [Unicode] stream into a Basic Text stream in a manner than always
succeeds but potentially loses information:
 - If the stream starts with U+FEFF, remove it.
 - Perform [String Conversion, Lossy].
 - If the stream is non-empty and doesn't end with U+A, append a U+A.

### Stream Conversion, Strict

To convert a [Unicode] stream into a Basic Text stream in a manner than never
loses information but may fail:
 - When *BOM Compatibility* is enabled, insert a U+FEFF at the beginning of the
   stream.
 - When *CRLF Compatibility* is enabled, replace any U+A with U+D U+A.
 - Perform [String Conversion, Strict].
 - If the stream is non-empty and doesn't end with U+A, error with
   "Basic Text stream must be empty or end with newline".

[Pre-NFC Table]: #pre-nfc-table
[Main Table]: #main-table
[String Conversion, Lossy]: #string-conversion-lossy
[String Conversion, Strict]: #string-conversion-strict

## TODO

TODO: `canonical_combining_class` doesn't know about the astral compositions
like U+11099 U+110BA => U+1109A. Restrict non-starters of that form too?

TODO: Validate/normalize [BiDi Controls]?

TODO: Validate [variation sequences]?

[NFC]: https://unicode.org/reports/tr15/#Norm_Forms
[Stream-Safe]: https://unicode.org/reports/tr15/#Stream_Safe_Text_Format
[Stream-Safe Text Process (UAX15-D4)]: https://unicode.org/reports/tr15/#UAX15-D4
[Standardized Variant]: https://www.unicode.org/Public/UNIDATA/StandardizedVariants.txt
[Normalization Process for Stabilized Strings]: https://unicode.org/reports/tr15/#Normalization_Process_for_Stabilized_Strings
[Noncharacters]: http://www.unicode.org/faq/private_use.html#noncharacters
[Deprecated Format Characters]: https://www.unicode.org/versions/Unicode13.0.0/ch23.pdf#G19593
[Private-Use Characters]: http://www.unicode.org/faq/private_use.html#private_use
[Tag Characters]: https://www.unicode.org/versions/Unicode13.0.0/ch23.pdf#G30110
[Restricted Text]: RestrictedText.md
[Unicode]: Unicode.md
[CJK Compatibility Ideographs]: http://www.unicode.org/versions/latest/ch23.pdf#G19053
[BiDi Controls]: https://unicode.org/reports/tr9/
[variation sequences]: http://unicode.org/faq/vs.html#3