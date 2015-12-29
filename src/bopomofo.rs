enum Bopomofo {
    Consonant(u16),
    Medial(u16),
    Rhyme(u16),
    Tone(u16),
}

const BOPOMOFO_CONSONANT_MASK: u16 = 0x1e00;
const BOPOMOFO_MEDIAL_MASK: u16    = 0x0180;
const BOPOMOFO_RHYME_MASK: u16     = 0x0078;
const BOPOMOFO_TONE_MASK: u16      = 0x0007;

const BOPOMOFO_CONSONANT_FIRST: char = 'ㄅ';
const BOPOMOFO_CONSONANT_LAST: char = 'ㄙ';

const BOPOMOFO_MEDIAL_FIRST: char = 'ㄧ';
const BOPOMOFO_MEDIAL_LAST: char = 'ㄩ';

const BOPOMOFO_RHYME_FIRST: char = 'ㄚ';
const BOPOMOFO_RHYME_LAST: char  = 'ㄦ';

const BOPOMOFO_B: u16  = 0x0200; // ㄅ
const BOPOMOFO_P: u16  = 0x0300; // ㄆ
const BOPOMOFO_M: u16  = 0x0400; // ㄇ
const BOPOMOFO_F: u16  = 0x0500; // ㄈ
const BOPOMOFO_D: u16  = 0x0600; // ㄉ
const BOPOMOFO_T: u16  = 0x0700; // ㄊ
const BOPOMOFO_N: u16  = 0x0800; // ㄋ
const BOPOMOFO_L: u16  = 0x0900; // ㄌ
const BOPOMOFO_G: u16  = 0x0a00; // ㄍ
const BOPOMOFO_K: u16  = 0x0b00; // ㄎ
const BOPOMOFO_H: u16  = 0x0c00; // ㄏ
const BOPOMOFO_J: u16  = 0x0d00; // ㄐ
const BOPOMOFO_Q: u16  = 0x0e00; // ㄑ
const BOPOMOFO_X: u16  = 0x0f00; // ㄒ
const BOPOMOFO_ZH: u16 = 0x1000; // ㄓ
const BOPOMOFO_CH: u16 = 0x1100; // ㄔ
const BOPOMOFO_SH: u16 = 0x1200; // ㄕ
const BOPOMOFO_R: u16  = 0x1300; // ㄖ
const BOPOMOFO_Z: u16  = 0x1400; // ㄗ
const BOPOMOFO_C: u16  = 0x1500; // ㄘ
const BOPOMOFO_S: u16  = 0x1600; // ㄙ

const BOPOMOFO_Y: u16   = 0x0008; // ㄚ
const BOPOMOFO_O: u16   = 0x0009; // ㄛ
const BOPOMOFO_E: u16   = 0x000a; // ㄜ
const BOPOMOFO_EH: u16  = 0x000b; // ㄝ
const BOPOMOFO_AI: u16  = 0x000c; // ㄞ
const BOPOMOFO_EI: u16  = 0x000d; // ㄟ
const BOPOMOFO_AU: u16  = 0x000e; // ㄠ
const BOPOMOFO_OU: u16  = 0x000f; // ㄡ
const BOPOMOFO_AN: u16  = 0x0010; // ㄢ
const BOPOMOFO_EN: u16  = 0x0020; // ㄣ
const BOPOMOFO_ANG: u16 = 0x0030; // ㄤ
const BOPOMOFO_ENG: u16 = 0x0040; // ㄥ
const BOPOMOFO_ER: u16  = 0x0050; // ㄦ

const BOPOMOFO_I: u16  = 0x0080; // ㄧ
const BOPOMOFO_U: u16  = 0x0090; // ㄨ
const BOPOMOFO_IU: u16 = 0x0100; // ㄩ

const BOPOMOFO_LIGHT_TONE: u16  = 0x0001; // ˙
const BOPOMOFO_SECOND_TONE: u16 = 0x0002; // ˊ
const BOPOMOFO_THIRD_TONE: u16  = 0x0003; // ˇ
const BOPOMOFO_FOURTH_TONE: u16 = 0x0004; // ˋ
