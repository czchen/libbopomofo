use std::cmp;
use std::fmt;

type phone = u16;

enum Bopomofo {
    Consonant(phone),
    Medial(phone),
    Rhyme(phone),
    Tone(phone),
}

impl Bopomofo {
    fn get_bopomofo(&self) -> phone {
        match self {
            &Bopomofo::Consonant(x) => x,
            &Bopomofo::Medial(x) => x,
            &Bopomofo::Rhyme(x) => x,
            &Bopomofo::Tone(x) => x,
        }
    }
}

impl cmp::PartialEq for Bopomofo {
    fn eq(&self, rhs: &Bopomofo) -> bool {
        self.get_bopomofo() == rhs.get_bopomofo()
    }
}

impl fmt::Debug for Bopomofo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bopomofo({})", self.get_bopomofo())
    }
}

// const BOPOMOFO_CONSONANT_MASK: u16 = 0x1e00;
// const BOPOMOFO_MEDIAL_MASK: u16    = 0x0180;
// const BOPOMOFO_RHYME_MASK: u16     = 0x0078;
// const BOPOMOFO_TONE_MASK: u16      = 0x0007;

const BOPOMOFO_CONSONANT_SHIFT: usize = 9;
const BOPOMOFO_MEDIAL_SHIFT: usize    = 6;
const BOPOMOFO_RHYME_SHIFT: usize     = 3;
const BOPOMOFO_TONE_SHIFT: usize      = 0;

const BOPOMOFO_B: phone  = 0x0001 << BOPOMOFO_CONSONANT_SHIFT; // ㄅ
const BOPOMOFO_P: phone  = 0x0002 << BOPOMOFO_CONSONANT_SHIFT; // ㄆ
const BOPOMOFO_M: phone  = 0x0003 << BOPOMOFO_CONSONANT_SHIFT; // ㄇ
const BOPOMOFO_F: phone  = 0x0004 << BOPOMOFO_CONSONANT_SHIFT; // ㄈ
const BOPOMOFO_D: phone  = 0x0005 << BOPOMOFO_CONSONANT_SHIFT; // ㄉ
const BOPOMOFO_T: phone  = 0x0006 << BOPOMOFO_CONSONANT_SHIFT; // ㄊ
const BOPOMOFO_N: phone  = 0x0007 << BOPOMOFO_CONSONANT_SHIFT; // ㄋ
const BOPOMOFO_L: phone  = 0x0008 << BOPOMOFO_CONSONANT_SHIFT; // ㄌ
const BOPOMOFO_G: phone  = 0x0009 << BOPOMOFO_CONSONANT_SHIFT; // ㄍ
const BOPOMOFO_K: phone  = 0x000a << BOPOMOFO_CONSONANT_SHIFT; // ㄎ
const BOPOMOFO_H: phone  = 0x000b << BOPOMOFO_CONSONANT_SHIFT; // ㄏ
const BOPOMOFO_J: phone  = 0x000c << BOPOMOFO_CONSONANT_SHIFT; // ㄐ
const BOPOMOFO_Q: phone  = 0x000d << BOPOMOFO_CONSONANT_SHIFT; // ㄑ
const BOPOMOFO_X: phone  = 0x000e << BOPOMOFO_CONSONANT_SHIFT; // ㄒ
const BOPOMOFO_ZH: phone = 0x000f << BOPOMOFO_CONSONANT_SHIFT; // ㄓ
const BOPOMOFO_CH: phone = 0x0010 << BOPOMOFO_CONSONANT_SHIFT; // ㄔ
const BOPOMOFO_SH: phone = 0x0011 << BOPOMOFO_CONSONANT_SHIFT; // ㄕ
const BOPOMOFO_R: phone  = 0x0012 << BOPOMOFO_CONSONANT_SHIFT; // ㄖ
const BOPOMOFO_Z: phone  = 0x0013 << BOPOMOFO_CONSONANT_SHIFT; // ㄗ
const BOPOMOFO_C: phone  = 0x0014 << BOPOMOFO_CONSONANT_SHIFT; // ㄘ
const BOPOMOFO_S: phone  = 0x0015 << BOPOMOFO_CONSONANT_SHIFT; // ㄙ

const BOPOMOFO_Y: phone   = 0x0001 << BOPOMOFO_RHYME_SHIFT; // ㄚ
const BOPOMOFO_O: phone   = 0x0002 << BOPOMOFO_RHYME_SHIFT; // ㄛ
const BOPOMOFO_E: phone   = 0x0003 << BOPOMOFO_RHYME_SHIFT; // ㄜ
const BOPOMOFO_EH: phone  = 0x0004 << BOPOMOFO_RHYME_SHIFT; // ㄝ
const BOPOMOFO_AI: phone  = 0x0005 << BOPOMOFO_RHYME_SHIFT; // ㄞ
const BOPOMOFO_EI: phone  = 0x0006 << BOPOMOFO_RHYME_SHIFT; // ㄟ
const BOPOMOFO_AU: phone  = 0x0007 << BOPOMOFO_RHYME_SHIFT; // ㄠ
const BOPOMOFO_OU: phone  = 0x0008 << BOPOMOFO_RHYME_SHIFT; // ㄡ
const BOPOMOFO_AN: phone  = 0x0009 << BOPOMOFO_RHYME_SHIFT; // ㄢ
const BOPOMOFO_EN: phone  = 0x000a << BOPOMOFO_RHYME_SHIFT; // ㄣ
const BOPOMOFO_ANG: phone = 0x000b << BOPOMOFO_RHYME_SHIFT; // ㄤ
const BOPOMOFO_ENG: phone = 0x000c << BOPOMOFO_RHYME_SHIFT; // ㄥ
const BOPOMOFO_ER: phone  = 0x000d << BOPOMOFO_RHYME_SHIFT; // ㄦ

const BOPOMOFO_I: phone  = 0x0001 << BOPOMOFO_MEDIAL_SHIFT; // ㄧ
const BOPOMOFO_U: phone  = 0x0002 << BOPOMOFO_MEDIAL_SHIFT; // ㄨ
const BOPOMOFO_IU: phone = 0x0003 << BOPOMOFO_MEDIAL_SHIFT; // ㄩ

const BOPOMOFO_LIGHT_TONE: phone  = 0x0001 << BOPOMOFO_TONE_SHIFT; // ˙
const BOPOMOFO_SECOND_TONE: phone = 0x0002 << BOPOMOFO_TONE_SHIFT; // ˊ
const BOPOMOFO_THIRD_TONE: phone  = 0x0003 << BOPOMOFO_TONE_SHIFT; // ˇ
const BOPOMOFO_FOURTH_TONE: phone = 0x0004 << BOPOMOFO_TONE_SHIFT; // ˋ

fn convert_to_bopomofo(ch: char) -> Option<Bopomofo> {
    if 'ㄅ' <= ch && ch <= 'ㄙ' {
        Some(Bopomofo::Consonant((ch as phone - 'ㄅ' as phone + 1) << BOPOMOFO_CONSONANT_SHIFT))

    } else if 'ㄚ' <= ch && ch <= 'ㄦ' {
        let diff = (ch as phone - 'ㄚ' as phone) << BOPOMOFO_RHYME_SHIFT;
        Some(Bopomofo::Rhyme(BOPOMOFO_Y + diff))

    } else if 'ㄧ' <= ch && ch <= 'ㄩ' {
        let diff = (ch as phone - 'ㄧ' as phone) << BOPOMOFO_MEDIAL_SHIFT;
        Some(Bopomofo::Medial(BOPOMOFO_I + diff))

    } else {
        match ch {
            '˙' => Some(Bopomofo::Tone(BOPOMOFO_LIGHT_TONE)),
            'ˊ' => Some(Bopomofo::Tone(BOPOMOFO_SECOND_TONE)),
            'ˇ' => Some(Bopomofo::Tone(BOPOMOFO_THIRD_TONE)),
            'ˋ' => Some(Bopomofo::Tone(BOPOMOFO_FOURTH_TONE)),
            _ => None,
        }
    }
}

#[test]
fn test_convert_to_bopomofo() {
    assert_eq!(convert_to_bopomofo('ㄅ'), Some(Bopomofo::Consonant(BOPOMOFO_B)));
    assert_eq!(convert_to_bopomofo('ㄆ'), Some(Bopomofo::Consonant(BOPOMOFO_P)));
    assert_eq!(convert_to_bopomofo('ㄇ'), Some(Bopomofo::Consonant(BOPOMOFO_M)));
    assert_eq!(convert_to_bopomofo('ㄈ'), Some(Bopomofo::Consonant(BOPOMOFO_F)));
    assert_eq!(convert_to_bopomofo('ㄉ'), Some(Bopomofo::Consonant(BOPOMOFO_D)));
    assert_eq!(convert_to_bopomofo('ㄊ'), Some(Bopomofo::Consonant(BOPOMOFO_T)));
    assert_eq!(convert_to_bopomofo('ㄋ'), Some(Bopomofo::Consonant(BOPOMOFO_N)));
    assert_eq!(convert_to_bopomofo('ㄌ'), Some(Bopomofo::Consonant(BOPOMOFO_L)));
    assert_eq!(convert_to_bopomofo('ㄍ'), Some(Bopomofo::Consonant(BOPOMOFO_G)));
    assert_eq!(convert_to_bopomofo('ㄎ'), Some(Bopomofo::Consonant(BOPOMOFO_K)));
    assert_eq!(convert_to_bopomofo('ㄏ'), Some(Bopomofo::Consonant(BOPOMOFO_H)));
    assert_eq!(convert_to_bopomofo('ㄐ'), Some(Bopomofo::Consonant(BOPOMOFO_J)));
    assert_eq!(convert_to_bopomofo('ㄑ'), Some(Bopomofo::Consonant(BOPOMOFO_Q)));
    assert_eq!(convert_to_bopomofo('ㄒ'), Some(Bopomofo::Consonant(BOPOMOFO_X)));
    assert_eq!(convert_to_bopomofo('ㄓ'), Some(Bopomofo::Consonant(BOPOMOFO_ZH)));
    assert_eq!(convert_to_bopomofo('ㄔ'), Some(Bopomofo::Consonant(BOPOMOFO_CH)));
    assert_eq!(convert_to_bopomofo('ㄕ'), Some(Bopomofo::Consonant(BOPOMOFO_SH)));
    assert_eq!(convert_to_bopomofo('ㄖ'), Some(Bopomofo::Consonant(BOPOMOFO_R)));
    assert_eq!(convert_to_bopomofo('ㄗ'), Some(Bopomofo::Consonant(BOPOMOFO_Z)));
    assert_eq!(convert_to_bopomofo('ㄘ'), Some(Bopomofo::Consonant(BOPOMOFO_C)));
    assert_eq!(convert_to_bopomofo('ㄙ'), Some(Bopomofo::Consonant(BOPOMOFO_S)));

    assert_eq!(convert_to_bopomofo('ㄚ'), Some(Bopomofo::Rhyme(BOPOMOFO_Y)));
    assert_eq!(convert_to_bopomofo('ㄛ'), Some(Bopomofo::Rhyme(BOPOMOFO_O)));
    assert_eq!(convert_to_bopomofo('ㄜ'), Some(Bopomofo::Rhyme(BOPOMOFO_E)));
    assert_eq!(convert_to_bopomofo('ㄝ'), Some(Bopomofo::Rhyme(BOPOMOFO_EH)));
    assert_eq!(convert_to_bopomofo('ㄞ'), Some(Bopomofo::Rhyme(BOPOMOFO_AI)));
    assert_eq!(convert_to_bopomofo('ㄟ'), Some(Bopomofo::Rhyme(BOPOMOFO_EI)));
    assert_eq!(convert_to_bopomofo('ㄠ'), Some(Bopomofo::Rhyme(BOPOMOFO_AU)));
    assert_eq!(convert_to_bopomofo('ㄡ'), Some(Bopomofo::Rhyme(BOPOMOFO_OU)));
    assert_eq!(convert_to_bopomofo('ㄢ'), Some(Bopomofo::Rhyme(BOPOMOFO_AN)));
    assert_eq!(convert_to_bopomofo('ㄣ'), Some(Bopomofo::Rhyme(BOPOMOFO_EN)));
    assert_eq!(convert_to_bopomofo('ㄤ'), Some(Bopomofo::Rhyme(BOPOMOFO_ANG)));
    assert_eq!(convert_to_bopomofo('ㄥ'), Some(Bopomofo::Rhyme(BOPOMOFO_ENG)));
    assert_eq!(convert_to_bopomofo('ㄦ'), Some(Bopomofo::Rhyme(BOPOMOFO_ER)));

    assert_eq!(convert_to_bopomofo('ㄧ'), Some(Bopomofo::Medial(BOPOMOFO_I)));
    assert_eq!(convert_to_bopomofo('ㄨ'), Some(Bopomofo::Medial(BOPOMOFO_U)));
    assert_eq!(convert_to_bopomofo('ㄩ'), Some(Bopomofo::Medial(BOPOMOFO_IU)));

    assert_eq!(convert_to_bopomofo('˙'), Some(Bopomofo::Tone(BOPOMOFO_LIGHT_TONE)));
    assert_eq!(convert_to_bopomofo('ˊ'), Some(Bopomofo::Tone(BOPOMOFO_SECOND_TONE)));
    assert_eq!(convert_to_bopomofo('ˇ'), Some(Bopomofo::Tone(BOPOMOFO_THIRD_TONE)));
    assert_eq!(convert_to_bopomofo('ˋ'), Some(Bopomofo::Tone(BOPOMOFO_FOURTH_TONE)));

    assert_eq!(convert_to_bopomofo('A'), None);
}
