fn string_to_number(s: &str) -> i32 {
  let s_int: i32 = s.to_string().parse::<i32>().unwrap();
    s_int
}

#[cfg(test)]
mod tests {
    use super::string_to_number;
    
    #[test]
    fn returns_expected() {
      assert_eq!(string_to_number("1234"), 1234);
      assert_eq!(string_to_number("605"), 605);
      assert_eq!(string_to_number("1405"), 1405);
      assert_eq!(string_to_number("-7"), -7);
    }
}