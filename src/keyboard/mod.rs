use glfw::Key;
use std::collections::HashMap;

pub mod azerty;

const CHARS_IN_ORDER : & 'static [& 'static str] =
&[
    "<Esc>",
    "<F1>",
    "<F2>",
    "<F3>",
    "<F4>",
    "<F5>",
    "<F6>",
    "<F7>",
    "<F8>",
    "<F9>",
    "<F10>",
    "<F11>",
    "<F12>",
    "<Insert>",
    "<Del>",
    "<BS>",
    "<Beg>",
    "<End>",
    "<PUp>",
    "<PDown>",
    "<Left>",
    "<Down>",
    "<Up>",
    "<Right>",
    "<CR>",
    "<Space>",

    // num bar with shift
    "²",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    "°",
    "+",
    

    // num bar alone
    "&",
    "é",
    "\"",
    "'",
    "(",
    "-",
    "è",
    "_",
    "ç",
    "à",
    ")",
    "=",

    // num bar with right-alt
    "¹",
    "~",
    "#",
    "{",
    "[",
    "|",
    "`",
    "\\",
    "^",
    "@",
    "]",
    "}",


    "a",
    "z",
    "e",
    "r",
    "t",
    "y",
    "u",
    "i",
    "o",
    "p",
    "^",
    "$",
    "q",
    "s",
    "d",
    "f",
    "g",
    "h",
    "j",
    "k",
    "l",
    "m",
    "ù",
    "*",
    "w",
    "x",
    "c",
    "v",
    "b",
    "n",
    ",",
    ";",
    ":",
    "!",
    "¨",
    "£",
    "%",
    "µ",
    "?",
    ".",
    "/",
    "§",
    "<Less>",
    "<More>",
    "<Tab>",

    "0",
    "1",
    "2",
    "3",
    "-",
    "4",
    "5",
    "6",
    "*",
    "7",
    "8",
    "9",
    "/",
    "A",
    "Z",
    "E",
    "R",
    "T",
    "Y",
    "U",
    "I",
    "O",
    "P",
    "Q",
    "S",
    "D",
    "F",
    "G",
    "H",
    "J",
    "K",
    "L",
    "M",
    "W",
    "X",
    "C",
    "V",
    "B",
    "N",

];


#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Mod {
    Shift,
    Control,
    Alt,
    AltGr,
}

impl Into<u8> for Mod {
    fn into(self) -> u8 {
        match self {
            Mod::Shift   => 0b0001,
            Mod::Control => 0b0010,
            Mod::Alt     => 0b0100,
            Mod::AltGr   => 0b1000,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct ModSet(u8);
impl ModSet {
    pub const fn empty() -> ModSet { ModSet(0) }
    pub const fn shift() -> ModSet { ModSet(0b0001) }
    pub const fn control() -> ModSet { ModSet(0b0010) }
    pub const fn alt() -> ModSet { ModSet(0b0100) }
    pub const fn altgr() -> ModSet { ModSet(0b1000) }

    pub fn is_set(&self, m:Mod) -> bool {
        let v : u8 = m.into();
        (self.0 & v) != 0u8
    }
    pub fn set(&mut self, m:Mod) {
        let v : u8 = m.into();
        self.0 |= v
    }
    pub fn clear(&mut self, m:Mod) {
        let v : u8 = m.into();
        self.0 &= !v
    }
    pub fn superset(&self, set:ModSet) -> bool { (self.0 & set.0) == set.0 }
    pub fn subset(&self, set:ModSet) -> bool { set.superset(*self) }
}

impl From<Mod> for ModSet {
    fn from(m:Mod) -> ModSet {
        let mut ret = ModSet::empty();
        ret.set(m);
        ret
    }
}

pub struct KeyboardLayout {
    pub(self) map:HashMap<(Key, ModSet), CharKey>,
}

impl KeyboardLayout {
    pub fn translate(&self, key:&(Key, ModSet)) -> Option<CharKey> {
        println!("translating {:?}", key);

        let ck = self.map.get(&key).map(|&s| s);
        if ck.is_some() { return ck }

        let mut set = key.1.clone();
        for &modif in &[Mod::Alt, Mod::Control, Mod::AltGr, Mod::Shift] {
            set.clear(modif);
            let key = self.map.get(&(key.0.clone(), set)).map(|&s| s);
            if key.is_some() { return key }
        }

        return None
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CharKey {
    Char(char),
    Special(u32),
}

impl From<&str> for CharKey {
    fn from(s:&str) -> CharKey {
        if s.len() == 1 {
            CharKey::Char(s.chars().next().unwrap())
        } else {
            let code : u32 = match s {
                "<Esc>" => 0,
                "<F1>" => 1,
                "<F2>" => 2,
                "<F3>" => 3,
                "<F4>" => 4,
                "<F5>" => 5,
                "<F6>" => 6,
                "<F7>" => 7,
                "<F8>" => 8,
                "<F9>" => 9,
                "<F10>" => 10,
                "<F11>" => 11,
                "<F12>" => 12,
                "<Insert>" => 13,
                "<Del>" => 14,
                "<BS>" => 15,
                "<Beg>" => 16,
                "<End>" => 17,
                "<PUp>" => 18,
                "<PDown>" => 19,
                "<Left>" => 20,
                "<Down>" => 21,
                "<Up>" => 22,
                "<Right>" => 23,
                "<CR>" => 24,
                "<Space>" => 25,
                "<Less>" => 26,
                "<More>" => 27,
                "<Tab>" => 28,
                _ => 1000,
            };

            if code == 25 {
                CharKey::Char(' ')
            } else if code == 26 {
                CharKey::Char('<')
            } else if code == 27 {
                CharKey::Char('>')
            } else {
                CharKey::Special(code)
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct CharKeyMod {
    pub key:CharKey,
    pub mods:ModSet,
}

impl From<&str> for CharKeyMod {
    fn from(s:&str) -> CharKeyMod {
        let mut set = ModSet::empty();
        if s.len() == 1 {
            if s.chars().next().unwrap().is_ascii_uppercase() {
                set.set(Mod::Shift)
            }

            CharKeyMod {
                key:s.into(),
                mods:set,
            }
        } else {
            let mut chars = s.chars();
            let mut string = String::new();

            chars.next();

            while let Some(mut c) = chars.next() {
                if c == 'S' || c == 'C' || c == 'A' {
                    let nc = chars.by_ref().next().unwrap();
                    if nc == '-' {
                        set.set(match c {
                            'S' => Mod::Shift,
                            'C' => Mod::Control,
                            'A' => Mod::Alt,
                            _ => unreachable!(),
                        });
                        continue
                    } else {
                        string.push(c);
                        c = nc;
                    }
                }

                if c == '>' { break }

                string.push(c)
            }

            if string.len() != 1 {
                string = format!("<{}>", string);
            } else {
                if s.chars().next().unwrap().is_ascii_uppercase() {
                    set.set(Mod::Shift)
                }
            }

            println!("WE TRANSLATED {}", string);
            let sref : &str = string.as_ref();
            CharKeyMod {
                key:sref.into(),
                mods:set,
            }
        }
    }
}
