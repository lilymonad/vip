use luminance_glfw::{GlfwSurface, Surface, WindowEvent, Action, Key};
use std::{rc::Rc, collections::{HashMap, HashSet}};
use crate::keyboard::{ModSet, KeyboardLayout, CharKeyMod, CharKey, Mod, azerty};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Mode {
    Normal,
    Command,
    Insertion,
    Visual,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct KeySequence {
    seq:Vec<CharKeyMod>,
}

impl From<&str> for KeySequence {
    fn from(s:&str) -> Self {
        let mut chars = s.chars();
        let mut seq : Vec<CharKeyMod> = Vec::new();

        while let Some(c) = chars.next() {
            if c == '<' {
                let mut s = '<'.to_string();
                let chars = chars.by_ref();
                while let Some(c) = chars.next() {
                    if c == '>' {
                        break;
                    }
                    s.push(c);
                }

                s.push('>');
                seq.push(CharKeyMod::from(s.as_ref()));
            } else {
                seq.push(CharKeyMod::from(c.to_string().as_ref()));
            }
        }

        Self { seq }
    }
}

pub type UiCommand<T> = dyn Fn(&mut Ui<T>, &mut T, &Vec<&str>);
pub type UiVerb<T> = dyn Fn(&mut Ui<T>, &mut T, Option<&HashSet<(usize, usize)>>);
pub type UiObject<T> = dyn Fn(&mut Ui<T>, &T, &mut HashSet<(usize, usize)>);
pub type UiCharProcessor<T> = dyn Fn(&mut Ui<T>, &mut T, CharKeyMod);

pub struct Ui<T> {
    commands: HashMap<String, Rc<UiCommand<T>>>,
    verbs: HashMap<CharKeyMod, (bool, Rc<UiVerb<T>>)>,
    objects: HashMap<CharKeyMod, Rc<UiObject<T>>>,
    char_processor: Rc<UiCharProcessor<T>>,

    bindings: HashMap<(CharKeyMod, Mode), KeySequence>,
    modset:ModSet,

    window_event_listener: Option<Rc<dyn Fn(&mut T, WindowEvent)>>,
    // buffer for storing unprocessed chars waiting
    buffer: String,

    // typed verb waiting for an object to come (if transitive)
    verb: Option<(usize, Rc<UiVerb<T>>)>,

    mode: Mode,
    running: bool,
    cursor: (usize, usize),
    saved_cursor: (usize, usize),
    layout:KeyboardLayout,
}

impl<T> Ui<T> {
    pub fn new<F : Fn(&mut Ui<T>, &mut T, CharKeyMod) + 'static>(f:F) -> Ui<T> {
        Ui {
            layout: azerty::layout(),
            modset: ModSet::empty(),
            commands: HashMap::new(),
            verbs: HashMap::new(),
            objects: HashMap::new(),
            bindings: HashMap::new(),

            window_event_listener: None,

            // processor of Insertion mode characters
            char_processor: Rc::new(f),

            buffer: String::new(),
            verb: None,

            mode: Mode::Normal,
            running: true,
            cursor: (0, 0),
            saved_cursor: (0, 0),
        }
    }

    pub fn input(&mut self, glfw: &mut GlfwSurface, env: &mut T) -> bool {
        for evt in glfw.poll_events() {
            match evt {
                WindowEvent::Close => self.running = false,

                WindowEvent::Key(Key::Backspace, _, Action::Press, _)
                    if self.mode == Mode::Command => {
                        self.buffer.pop();
                },

                // Every other key pressed will update the buffer and the state of the Ui.
                WindowEvent::Key(k, _, act, _) if act != Action::Release => {
                    match k {
                        Key::LeftShift | Key::RightShift => self.modset.set(Mod::Shift),
                        Key::LeftControl | Key::RightControl => self.modset.set(Mod::Control),
                        Key::LeftAlt => self.modset.set(Mod::Alt),
                        Key::RightAlt => self.modset.set(Mod::AltGr),
                        _ => {},
                    }

                    if let Some(code) = self.layout.translate(&(k, self.modset)).clone() {
                        let key_mod = CharKeyMod { key: code, mods: self.modset };
                        let pair = (key_mod, self.mode);

                        if let Some(KeySequence { seq }) = self.bindings.get(&pair) {
                            for CharKeyMod { key, mods } in seq.clone() {
                                self.perform_char_mod(env, key, mods);
                            }
                        } else {
                            self.perform_char_mod(env, code, self.modset)
                        }
                    }
                },

                WindowEvent::Key(k, _, Action::Release, _) => {
                    match k {
                        Key::LeftShift | Key::RightShift => self.modset.clear(Mod::Shift),
                        Key::LeftControl | Key::RightControl => self.modset.clear(Mod::Control),
                        Key::LeftAlt => self.modset.clear(Mod::Alt),
                        Key::RightAlt => self.modset.clear(Mod::AltGr),
                        _ => {},
                    }
                },

                e => {
                    let _ = self.window_event_listener.as_ref().map(|f| {
                        let f = f.clone();
                        f(env, e)
                    });
                },
            }
        }

        self.running
    }

    fn launch_command(&mut self, env:&mut T, command:String) {
        let mut words = command.split_whitespace();

        let name = words.next().unwrap();
        let args = words.collect();

        if let Some(command) = self.commands.get(name) {
            let command = command.clone();
            command(self, env, &args);
        }
    }

    fn perform_char_mod(&mut self, env:&mut T, c:CharKey, mods:ModSet) {
        match c {

            // Special(0) == escape. It always comes back to normal mode and clears any buffered
            // input (command buffer, normal verb buffer ...)
            CharKey::Special(0) => {
                self.cursor = self.saved_cursor;
                self.set_mode(Mode::Normal);
                self.buffer.clear();
                self.verb = None;
                // We added the possibility to bind Escape key to a verb
                if let Some((_, action)) = self.verbs.get(&CharKeyMod { key:c, mods }) {
                    (action.clone())(self, env, None);
                }
            },

            // Return pressed while in Command mode triggers the command, and returns to normal
            // mode
            CharKey::Special(24) if self.mode == Mode::Command => {
                let s = std::mem::replace(&mut self.buffer, String::new());
                self.launch_command(env, s);
                self.set_mode(Mode::Normal);
            },
            // any character in insertion mode
            c if self.mode == Mode::Insertion => {
                let processor = self.char_processor.clone();
                processor(self, env, CharKeyMod { key:c, mods })
            },

            // number in non-insertion mode
            CharKey::Char(c) if c.is_ascii_digit() => {
                self.buffer.push(c);
            },

            // any character in normal mode
            c if self.mode == Mode::Normal || self.mode == Mode::Visual => {
                // parse count
                let count = if self.buffer.len() == 0 { 1 }
                            else { self.buffer.parse().unwrap() };

                self.buffer.clear();

                // if we already had a verb
                // then the char is an object
                if let Some((n, v)) = self.verb.take() {
                    let verb = v.clone();
                    let obj = CharKeyMod { key:c, mods };
                    let object = self.objects.get(&obj);

                    if let Some(object) = object {
                        let object = object.clone();
                        let mut positions = HashSet::new();

                        // for each asked application
                        for _ in 0..n {
                            positions.clear();

                            // move count times
                            for _ in 0..count {
                                object(self, env, &mut positions);
                            }

                            // apply verb on given positions
                            verb(self, env, Some(&positions));
                        }
                    }
                } else { // else, we have a verb, we need to check for transitivity


                    // check if verb exist
                    let verb = CharKeyMod { key:c, mods };
                    let object = verb.clone();
                    if let Some((is_transitive, action)) = self.verbs.get(&verb) {

                        if *is_transitive {
                            self.verb = Some((count, action.clone()));
                        } else {
                            let act = action.clone();
                            for _ in 0..count {
                                act(self, env, None);
                            }
                        }
                    } else if let Some(object) = self.objects.get(&object) {
                        let object = object.clone();
                        let mut v = HashSet::new();
                        for _ in 0..count {
                            object(self, env, &mut v);
                        }
                    }
                }
            },
            CharKey::Char(c) if self.mode == Mode::Command => {
                self.buffer.push(c);
            },
            _ => {},
        }
    }

    pub fn add_verb<B, F>(&mut self, verb:B, transitive:bool, f:F)
        where F : Fn(&mut Ui<T>, &mut T, Option<&HashSet<(usize, usize)>>) + 'static,
              B : Into<CharKeyMod>,
    {
        let _ = self.verbs.insert(verb.into(), (transitive, Rc::new(f)));
    }

    pub fn add_object<O, F>(&mut self, obj:O, f:F)
        where F : (Fn(&mut Ui<T>, &T, &mut HashSet<(usize, usize)>)) + 'static,
              O : Into<CharKeyMod>,
    {
        let _ = self.objects.insert(obj.into(), Rc::new(f));
    }

    pub fn add_command<S, F>(&mut self, name:S, f:F)
        where F : Fn(&mut Ui<T>, &mut T, &Vec<&str>) + 'static,
              S : Into<String>,
    {
        let _ = self.commands.insert(name.into(), Rc::new(f));
    }

    pub fn bind_key<K:Into<CharKeyMod>, S:Into<KeySequence>>(&mut self, k:K, mode:Mode, phrase:S) {
        let k = k.into();
        let s = phrase.into();
        self.bindings.insert((k, mode), s);
    }

    pub fn close(&mut self) {
        self.running = false
    }

    pub fn cursor(&self) -> (usize, usize) {
        self.cursor
    }

    pub fn wrapping_displace(&mut self, dx:isize, dy:isize, w:usize, h:usize) {
        self.cursor.0 = ((self.cursor.0 as isize).wrapping_add(dx) as usize).min(w - 1);
        self.cursor.1 = ((self.cursor.1 as isize).wrapping_add(dy) as usize).min(h - 1);
        if self.mode != Mode::Visual {
            self.saved_cursor = self.cursor
        }
    }

    pub fn displace(&mut self, dx:isize, dy:isize, w:usize, h:usize) {
        self.cursor.0 = ((self.cursor.0 as isize).saturating_add(dx) as usize).min(w - 1);
        self.cursor.1 = ((self.cursor.1 as isize).saturating_add(dy) as usize).min(h - 1);
        if self.mode != Mode::Visual {
            self.saved_cursor = self.cursor
        }
    }

    pub fn set_mode(&mut self, mode:Mode) {
        self.mode = mode
    }

    pub fn get_mode(&self) -> Mode {
        self.mode
    }

    pub fn get_selection(&self) -> ((usize, usize), (usize, usize)) {
        let (x1, y1) = self.cursor;
        let (x2, y2) = self.saved_cursor;
        ((x1.min(x2), y1.min(y2)), (x1.max(x2), y1.max(y2)))
    }

    pub fn set_window_event_listener<F:Fn(&mut T, WindowEvent) + 'static>(&mut self, f:Option<F>) {
        match f {
            Some(f) => {
                self.window_event_listener = Some(Rc::new(f));
            },
            None => self.window_event_listener = None,
        }
    }

    pub fn get_buffer(&self) -> &String {
        &self.buffer
    }
}
