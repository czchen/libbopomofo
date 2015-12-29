enum Bopomofo {
    Consonant(u16),
    Medial(u16),
    Rhyme(u16),
    Tone(u16),
}

fn convert_character_to_bopomofo(ch: char) -> Option<Bopomofo> {
    match ch {
        '˙' => Some(Bopomofo::Tone(1)),
        'ˊ' => Some(Bopomofo::Tone(2)),
        'ˇ' => Some(Bopomofo::Tone(3)),
        'ˋ' => Some(Bopomofo::Tone(4)),
        _ => None,
    }
}
