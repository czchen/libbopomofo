struct PreeditSlot {
    bopomofo: u16
}

struct Bopomofo {
    preedit_buffer: Vec<PreeditSlot>
}

impl Bopomofo {
    fn input(self, ch: char, pos: usize) {
    }
}
