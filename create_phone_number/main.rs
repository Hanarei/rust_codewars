fn create_phone_number(numbers: &[u8]) -> String {
    let mut result = String::new();
    for i in numbers{
        result.push_str(&i.to_string());
    }
    result.insert(0, '(');
    result.insert_str(4, ") ");
    result.insert(9, '-');
    
    result
}

#[test]
fn returns_expected() {
    for a in 0..=5 {
       for b in 6..=9 {
            for c in (0..=9).step_by(2) {
                let s = [a, b, c, b, c, a, c, b, a, a];
                assert_eq!(create_phone_number(&s), format!("({}{}{}) {}{}{}-{}{}{}{}", s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8], s[9]));
            }
        }
    }
}