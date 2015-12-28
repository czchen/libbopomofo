fn convert_to_bopomofo(ch: char) -> Option<u16> {
    match ch {
        'ˊ' => Some(1),
        'ˇ' => Some(2),
        'ˋ' => Some(3),
        '˙' => Some(4),
        _ => None,
    }
}

#[test]
fn test_convert_to_bopomofo() {
    assert_eq!(convert_to_bopomofo('a'), None);
    assert_eq!(convert_to_bopomofo('ˊ'), Some(1));
}
