use std::cmp;
use std::fmt;

pub type Phone = u16;

pub enum Bopomofo {
    Consonant(Phone),
    Medial(Phone),
    Rhyme(Phone),
    Tone(Phone),
}

impl Bopomofo {
    fn get_phone(&self) -> Phone {
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
        self.get_phone() == rhs.get_phone()
    }
}

impl fmt::Debug for Bopomofo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = match self {
            &Bopomofo::Consonant(_) => "Consonant",
            &Bopomofo::Medial(_) => "Medial",
            &Bopomofo::Rhyme(_) => "Rhyme",
            &Bopomofo::Tone(_) => "Tone",
        };

        write!(f, "{}({})", t, self.get_phone())
    }
}

const BOPOMOFO_CONSONANT_SHIFT: usize = 9;
const BOPOMOFO_MEDIAL_SHIFT: usize    = 6;
const BOPOMOFO_RHYME_SHIFT: usize     = 3;
const BOPOMOFO_TONE_SHIFT: usize      = 0;

const BOPOMOFO_CONSONANT_MASK: u16 = 0x001f << BOPOMOFO_CONSONANT_SHIFT;
const BOPOMOFO_MEDIAL_MASK: u16    = 0x0003 << BOPOMOFO_MEDIAL_SHIFT;
const BOPOMOFO_RHYME_MASK: u16     = 0x000f << BOPOMOFO_RHYME_SHIFT;
const BOPOMOFO_TONE_MASK: u16      = 0x0007 << BOPOMOFO_TONE_SHIFT;

pub const BOPOMOFO_B: Phone  = 0x0001 << BOPOMOFO_CONSONANT_SHIFT; // ㄅ
pub const BOPOMOFO_P: Phone  = 0x0002 << BOPOMOFO_CONSONANT_SHIFT; // ㄆ
pub const BOPOMOFO_M: Phone  = 0x0003 << BOPOMOFO_CONSONANT_SHIFT; // ㄇ
pub const BOPOMOFO_F: Phone  = 0x0004 << BOPOMOFO_CONSONANT_SHIFT; // ㄈ
pub const BOPOMOFO_D: Phone  = 0x0005 << BOPOMOFO_CONSONANT_SHIFT; // ㄉ
pub const BOPOMOFO_T: Phone  = 0x0006 << BOPOMOFO_CONSONANT_SHIFT; // ㄊ
pub const BOPOMOFO_N: Phone  = 0x0007 << BOPOMOFO_CONSONANT_SHIFT; // ㄋ
pub const BOPOMOFO_L: Phone  = 0x0008 << BOPOMOFO_CONSONANT_SHIFT; // ㄌ
pub const BOPOMOFO_G: Phone  = 0x0009 << BOPOMOFO_CONSONANT_SHIFT; // ㄍ
pub const BOPOMOFO_K: Phone  = 0x000a << BOPOMOFO_CONSONANT_SHIFT; // ㄎ
pub const BOPOMOFO_H: Phone  = 0x000b << BOPOMOFO_CONSONANT_SHIFT; // ㄏ
pub const BOPOMOFO_J: Phone  = 0x000c << BOPOMOFO_CONSONANT_SHIFT; // ㄐ
pub const BOPOMOFO_Q: Phone  = 0x000d << BOPOMOFO_CONSONANT_SHIFT; // ㄑ
pub const BOPOMOFO_X: Phone  = 0x000e << BOPOMOFO_CONSONANT_SHIFT; // ㄒ
pub const BOPOMOFO_ZH: Phone = 0x000f << BOPOMOFO_CONSONANT_SHIFT; // ㄓ
pub const BOPOMOFO_CH: Phone = 0x0010 << BOPOMOFO_CONSONANT_SHIFT; // ㄔ
pub const BOPOMOFO_SH: Phone = 0x0011 << BOPOMOFO_CONSONANT_SHIFT; // ㄕ
pub const BOPOMOFO_R: Phone  = 0x0012 << BOPOMOFO_CONSONANT_SHIFT; // ㄖ
pub const BOPOMOFO_Z: Phone  = 0x0013 << BOPOMOFO_CONSONANT_SHIFT; // ㄗ
pub const BOPOMOFO_C: Phone  = 0x0014 << BOPOMOFO_CONSONANT_SHIFT; // ㄘ
pub const BOPOMOFO_S: Phone  = 0x0015 << BOPOMOFO_CONSONANT_SHIFT; // ㄙ

pub const BOPOMOFO_Y: Phone   = 0x0001 << BOPOMOFO_RHYME_SHIFT; // ㄚ
pub const BOPOMOFO_O: Phone   = 0x0002 << BOPOMOFO_RHYME_SHIFT; // ㄛ
pub const BOPOMOFO_E: Phone   = 0x0003 << BOPOMOFO_RHYME_SHIFT; // ㄜ
pub const BOPOMOFO_EH: Phone  = 0x0004 << BOPOMOFO_RHYME_SHIFT; // ㄝ
pub const BOPOMOFO_AI: Phone  = 0x0005 << BOPOMOFO_RHYME_SHIFT; // ㄞ
pub const BOPOMOFO_EI: Phone  = 0x0006 << BOPOMOFO_RHYME_SHIFT; // ㄟ
pub const BOPOMOFO_AU: Phone  = 0x0007 << BOPOMOFO_RHYME_SHIFT; // ㄠ
pub const BOPOMOFO_OU: Phone  = 0x0008 << BOPOMOFO_RHYME_SHIFT; // ㄡ
pub const BOPOMOFO_AN: Phone  = 0x0009 << BOPOMOFO_RHYME_SHIFT; // ㄢ
pub const BOPOMOFO_EN: Phone  = 0x000a << BOPOMOFO_RHYME_SHIFT; // ㄣ
pub const BOPOMOFO_ANG: Phone = 0x000b << BOPOMOFO_RHYME_SHIFT; // ㄤ
pub const BOPOMOFO_ENG: Phone = 0x000c << BOPOMOFO_RHYME_SHIFT; // ㄥ
pub const BOPOMOFO_ER: Phone  = 0x000d << BOPOMOFO_RHYME_SHIFT; // ㄦ

pub const BOPOMOFO_I: Phone  = 0x0001 << BOPOMOFO_MEDIAL_SHIFT; // ㄧ
pub const BOPOMOFO_U: Phone  = 0x0002 << BOPOMOFO_MEDIAL_SHIFT; // ㄨ
pub const BOPOMOFO_IU: Phone = 0x0003 << BOPOMOFO_MEDIAL_SHIFT; // ㄩ

pub const BOPOMOFO_LIGHT_TONE: Phone  = 0x0001 << BOPOMOFO_TONE_SHIFT; // ˙
pub const BOPOMOFO_SECOND_TONE: Phone = 0x0002 << BOPOMOFO_TONE_SHIFT; // ˊ
pub const BOPOMOFO_THIRD_TONE: Phone  = 0x0003 << BOPOMOFO_TONE_SHIFT; // ˇ
pub const BOPOMOFO_FOURTH_TONE: Phone = 0x0004 << BOPOMOFO_TONE_SHIFT; // ˋ

pub fn convert_to_bopomofo(ch: char) -> Option<Bopomofo> {
    if 'ㄅ' <= ch && ch <= 'ㄙ' {
        Some(Bopomofo::Consonant((ch as Phone - 'ㄅ' as Phone + 1) << BOPOMOFO_CONSONANT_SHIFT))

    } else if 'ㄚ' <= ch && ch <= 'ㄦ' {
        Some(Bopomofo::Rhyme((ch as Phone - 'ㄚ' as Phone + 1) << BOPOMOFO_RHYME_SHIFT))

    } else if 'ㄧ' <= ch && ch <= 'ㄩ' {
        Some(Bopomofo::Medial((ch as Phone - 'ㄧ' as Phone + 1) << BOPOMOFO_MEDIAL_SHIFT))

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

pub fn merge_bopomofo_to_phone(phone: Phone, bopomofo: Bopomofo) -> Phone {
    let mask = match bopomofo {
        Bopomofo::Consonant(_) => BOPOMOFO_CONSONANT_MASK,
        Bopomofo::Medial(_) => BOPOMOFO_MEDIAL_MASK,
        Bopomofo::Rhyme(_) => BOPOMOFO_RHYME_MASK,
        Bopomofo::Tone(_) => BOPOMOFO_TONE_MASK,
    };

    phone & !mask | bopomofo.get_phone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_bopomofo_consonant() {
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
    }

    #[test]
    fn test_convert_to_bopomofo_medial() {
        assert_eq!(convert_to_bopomofo('ㄧ'), Some(Bopomofo::Medial(BOPOMOFO_I)));
        assert_eq!(convert_to_bopomofo('ㄨ'), Some(Bopomofo::Medial(BOPOMOFO_U)));
        assert_eq!(convert_to_bopomofo('ㄩ'), Some(Bopomofo::Medial(BOPOMOFO_IU)));
    }

    #[test]
    fn test_convert_to_bopomofo_rhyme() {
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
    }

    #[test]
    fn test_convert_to_bopomofo_tone() {
        assert_eq!(convert_to_bopomofo('˙'), Some(Bopomofo::Tone(BOPOMOFO_LIGHT_TONE)));
        assert_eq!(convert_to_bopomofo('ˊ'), Some(Bopomofo::Tone(BOPOMOFO_SECOND_TONE)));
        assert_eq!(convert_to_bopomofo('ˇ'), Some(Bopomofo::Tone(BOPOMOFO_THIRD_TONE)));
        assert_eq!(convert_to_bopomofo('ˋ'), Some(Bopomofo::Tone(BOPOMOFO_FOURTH_TONE)));
    }

    #[test]
    fn test_convert_to_bopomofo_others() {
        assert_eq!(convert_to_bopomofo('A'), None);
    }

    #[test]
    fn test_merge_bopomofo_to_phone_consonant() {
        let phone = BOPOMOFO_P | BOPOMOFO_O | BOPOMOFO_THIRD_TONE;
        assert_eq!(merge_bopomofo_to_phone(phone, Bopomofo::Consonant(BOPOMOFO_B)),
            BOPOMOFO_B | BOPOMOFO_O | BOPOMOFO_THIRD_TONE);
    }

    #[test]
    fn test_merge_bopomofo_to_phone_medial() {
        let phone = BOPOMOFO_P | BOPOMOFO_O | BOPOMOFO_THIRD_TONE;
        assert_eq!(merge_bopomofo_to_phone(phone, Bopomofo::Medial(BOPOMOFO_I)),
            BOPOMOFO_P | BOPOMOFO_I | BOPOMOFO_O | BOPOMOFO_THIRD_TONE);
    }

    #[test]
    fn test_merge_bopomofo_to_phone_rhyme() {
        let phone = BOPOMOFO_P | BOPOMOFO_O | BOPOMOFO_THIRD_TONE;
        assert_eq!(merge_bopomofo_to_phone(phone, Bopomofo::Rhyme(BOPOMOFO_AN)),
        BOPOMOFO_P | BOPOMOFO_AN | BOPOMOFO_THIRD_TONE);
    }

    #[test]
    fn test_merge_bopomofo_to_phone_tone() {
        let phone = BOPOMOFO_P | BOPOMOFO_O | BOPOMOFO_THIRD_TONE;
        assert_eq!(merge_bopomofo_to_phone(phone, Bopomofo::Tone(BOPOMOFO_THIRD_TONE)),
            BOPOMOFO_P | BOPOMOFO_O | BOPOMOFO_THIRD_TONE);
    }
}
