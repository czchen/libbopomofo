enum Bopomofo {
    Consonant(u16),
    Medial(u16),
    Rhyme(u16),
    Tone(u16),
}

const BOPOMOFO_CONSONANT_MASK: u16 = 0x1e00;

const BOPOMOFO_B: u16 =  0x0200; // ㄅ
const BOPOMOFO_P: u16 =  0x0300; // ㄆ
const BOPOMOFO_M: u16 =  0x0400; // ㄇ
const BOPOMOFO_F: u16 =  0x0500; // ㄈ
const BOPOMOFO_D: u16 =  0x0600; // ㄉ
const BOPOMOFO_T: u16 =  0x0700; // ㄊ
const BOPOMOFO_N: u16 =  0x0800; // ㄋ
const BOPOMOFO_L: u16 =  0x0900; // ㄌ
const BOPOMOFO_G: u16 =  0x0a00; // ㄍ
const BOPOMOFO_K: u16 =  0x0b00; // ㄎ
const BOPOMOFO_H: u16 =  0x0c00; // ㄏ
const BOPOMOFO_J: u16 =  0x0d00; // ㄐ
const BOPOMOFO_Q: u16 =  0x0e00; // ㄑ
const BOPOMOFO_X: u16 =  0x0f00; // ㄒ
const BOPOMOFO_ZH: u16 = 0x1000; // ㄓ
const BOPOMOFO_CH: u16 = 0x1100; // ㄔ
const BOPOMOFO_SH: u16 = 0x1200; // ㄕ
const BOPOMOFO_R: u16  = 0x1300; // ㄖ
const BOPOMOFO_Z: u16 =  0x1400; // ㄗ
const BOPOMOFO_C: u16 =  0x1500; // ㄘ
const BOPOMOFO_S: u16 =  0x1600; // ㄙ


const BOPOMOFO_MEDIAL_MASK: u16 = 0x0180;
const BOPOMOFO_RHYME_MASK: u16 = 0x0078;
const BOPOMOFO_TONE_MASK: u16 = 0x0007;

fn convert_character_to_bopomofo(ch: char) -> Option<Bopomofo> {
    match ch {
        'ㄅ' => Some(Bopomofo::Consonant(BOPOMOFO_B)),
        'ㄆ' => Some(Bopomofo::Consonant(BOPOMOFO_P)),
        'ㄇ' => Some(Bopomofo::Consonant(BOPOMOFO_M)),
        'ㄈ' => Some(Bopomofo::Consonant(BOPOMOFO_F)),
        'ㄉ' => Some(Bopomofo::Consonant(BOPOMOFO_D)),
        'ㄊ' => Some(Bopomofo::Consonant(BOPOMOFO_T)),
        'ㄋ' => Some(Bopomofo::Consonant(BOPOMOFO_N)),
        'ㄌ' => Some(Bopomofo::Consonant(BOPOMOFO_L)),
        'ㄍ' => Some(Bopomofo::Consonant(BOPOMOFO_G)),
        'ㄎ' => Some(Bopomofo::Consonant(BOPOMOFO_K)),
        'ㄏ' => Some(Bopomofo::Consonant(BOPOMOFO_H)),
        'ㄐ' => Some(Bopomofo::Consonant(BOPOMOFO_J)),
        'ㄑ' => Some(Bopomofo::Consonant(BOPOMOFO_Q)),
        'ㄒ' => Some(Bopomofo::Consonant(BOPOMOFO_X)),
        'ㄓ' => Some(Bopomofo::Consonant(BOPOMOFO_ZH)),
        'ㄔ' => Some(Bopomofo::Consonant(BOPOMOFO_CH)),
        'ㄕ' => Some(Bopomofo::Consonant(BOPOMOFO_SH)),
        'ㄖ' => Some(Bopomofo::Consonant(BOPOMOFO_R)),
        'ㄗ' => Some(Bopomofo::Consonant(BOPOMOFO_Z)),
        'ㄘ' => Some(Bopomofo::Consonant(BOPOMOFO_C)),
        'ㄙ' => Some(Bopomofo::Consonant(BOPOMOFO_S)),

        'ㄧ' => Some(Bopomofo::Medial(0x0080)),
        'ㄨ' => Some(Bopomofo::Medial(0x0090)),
        'ㄩ' => Some(Bopomofo::Medial(0x0100)),

        'ㄚ' => Some(Bopomofo::Rhyme(0x0008)),
        'ㄛ' => Some(Bopomofo::Rhyme(0x0009)),
        'ㄜ' => Some(Bopomofo::Rhyme(0x000a)),
        'ㄝ' => Some(Bopomofo::Rhyme(0x000b)),
        'ㄞ' => Some(Bopomofo::Rhyme(0x000c)),
        'ㄟ' => Some(Bopomofo::Rhyme(0x000d)),
        'ㄠ' => Some(Bopomofo::Rhyme(0x000e)),
        'ㄡ' => Some(Bopomofo::Rhyme(0x000f)),
        'ㄢ' => Some(Bopomofo::Rhyme(0x0010)),
        'ㄣ' => Some(Bopomofo::Rhyme(0x0020)),
        'ㄤ' => Some(Bopomofo::Rhyme(0x0030)),
        'ㄥ' => Some(Bopomofo::Rhyme(0x0040)),
        'ㄦ' => Some(Bopomofo::Rhyme(0x0050)),

        '˙' => Some(Bopomofo::Tone(0x0001)),
        'ˊ' => Some(Bopomofo::Tone(0x0002)),
        'ˇ' => Some(Bopomofo::Tone(0x0003)),
        'ˋ' => Some(Bopomofo::Tone(0x0004)),
        _ => None,
    }
}

fn convert_bopomofo_to_character(bopomofo: Bopomofo) -> Option<char> {
    match bopomofo {
        _ => None,
    }
}
