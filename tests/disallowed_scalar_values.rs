pub const DISALLOWED_SCALAR_VALUES: [char; 190] = [
    // All C0, U+7F, and C1 control codes other than U+A (newline),
    // U+9 (horizontal tab), U+D (carriage return), U+C (form feed),
    // U+7 (alert), U+1B (escape) and U+85 (NEL). Many of these are
    // nonetheless disallowed, but handled via different mechanisms.
    '\0',
    '\u{1}',
    '\u{2}',
    '\u{3}',
    '\u{4}',
    '\u{5}',
    '\u{6}',
    '\u{7}',
    '\u{8}',
    '\u{b}',
    '\u{e}',
    '\u{f}',
    '\u{10}',
    '\u{11}',
    '\u{12}',
    '\u{13}',
    '\u{14}',
    '\u{15}',
    '\u{16}',
    '\u{17}',
    '\u{18}',
    '\u{19}',
    '\u{1a}',
    '\u{1c}',
    '\u{1d}',
    '\u{1e}',
    '\u{1f}',
    '\u{7f}',
    '\u{80}',
    '\u{81}',
    '\u{82}',
    '\u{83}',
    '\u{84}',
    '\u{86}',
    '\u{87}',
    '\u{88}',
    '\u{89}',
    '\u{8a}',
    '\u{8b}',
    '\u{8c}',
    '\u{8d}',
    '\u{8e}',
    '\u{8f}',
    '\u{90}',
    '\u{91}',
    '\u{92}',
    '\u{93}',
    '\u{94}',
    '\u{95}',
    '\u{96}',
    '\u{97}',
    '\u{98}',
    '\u{99}',
    '\u{9a}',
    '\u{9b}',
    '\u{9c}',
    '\u{9d}',
    '\u{9e}',
    '\u{9f}',
    // Unassigned characters with replacements.
    '\u{9e4}',
    '\u{9e5}',
    '\u{a64}',
    '\u{a65}',
    '\u{ae4}',
    '\u{ae5}',
    '\u{b64}',
    '\u{b65}',
    '\u{be4}',
    '\u{be5}',
    '\u{c64}',
    '\u{c65}',
    '\u{ce4}',
    '\u{ce5}',
    '\u{d64}',
    '\u{d65}',
    // Khmer characters erroneously invented by Unicode.
    '\u{17b4}',
    '\u{17b5}',
    '\u{17d8}',
    // Bidirectional Format Characters.
    '\u{202a}',
    '\u{202b}',
    '\u{202c}',
    '\u{202d}',
    '\u{202e}',
    '\u{2066}',
    '\u{2067}',
    '\u{2068}',
    '\u{2069}',
    // Deprecated Format Characters
    '\u{206a}',
    '\u{206b}',
    '\u{206c}',
    '\u{206d}',
    '\u{206e}',
    '\u{206f}',
    // Unassigned characters with replacements.
    '\u{2072}',
    '\u{2073}',
    // Interlinear Annotations
    '\u{fff9}',
    '\u{fffa}',
    '\u{fffb}',
    // Object Replacement Character
    '\u{fffc}',
    // Language Tag
    '\u{e0001}',
    // Noncharacters
    '\u{fffe}',
    '\u{ffff}',
    '\u{1fffe}',
    '\u{1ffff}',
    '\u{2fffe}',
    '\u{2ffff}',
    '\u{3fffe}',
    '\u{3ffff}',
    '\u{4fffe}',
    '\u{4ffff}',
    '\u{5fffe}',
    '\u{5ffff}',
    '\u{6fffe}',
    '\u{6ffff}',
    '\u{7fffe}',
    '\u{7ffff}',
    '\u{8fffe}',
    '\u{8ffff}',
    '\u{9fffe}',
    '\u{9ffff}',
    '\u{afffe}',
    '\u{affff}',
    '\u{bfffe}',
    '\u{bffff}',
    '\u{cfffe}',
    '\u{cffff}',
    '\u{dfffe}',
    '\u{dffff}',
    '\u{efffe}',
    '\u{effff}',
    '\u{ffffe}',
    '\u{fffff}',
    '\u{10fffe}',
    '\u{10ffff}',
    '\u{fdd0}',
    '\u{fdd1}',
    '\u{fdd2}',
    '\u{fdd3}',
    '\u{fdd4}',
    '\u{fdd5}',
    '\u{fdd6}',
    '\u{fdd7}',
    '\u{fdd8}',
    '\u{fdd9}',
    '\u{fdda}',
    '\u{fddb}',
    '\u{fddc}',
    '\u{fddd}',
    '\u{fdde}',
    '\u{fddf}',
    '\u{fde0}',
    '\u{fde1}',
    '\u{fde2}',
    '\u{fde3}',
    '\u{fde4}',
    '\u{fde5}',
    '\u{fde6}',
    '\u{fde7}',
    '\u{fde8}',
    '\u{fde9}',
    '\u{fdea}',
    '\u{fdeb}',
    '\u{fdec}',
    '\u{fded}',
    '\u{fdee}',
    '\u{fdef}',
    '\u{1d455}',
    '\u{1d49d}',
    '\u{1d4a0}',
    '\u{1d4a1}',
    '\u{1d4a3}',
    '\u{1d4a4}',
    '\u{1d4a7}',
    '\u{1d4a8}',
    '\u{1d4ad}',
    '\u{1d4ba}',
    '\u{1d4bc}',
    '\u{1d4c4}',
    '\u{1d506}',
    '\u{1d50b}',
    '\u{1d50c}',
    '\u{1d515}',
    '\u{1d51d}',
    '\u{1d53a}',
    '\u{1d53f}',
    '\u{1d545}',
    '\u{1d547}',
    '\u{1d548}',
    '\u{1d549}',
    '\u{1d551}',
];
