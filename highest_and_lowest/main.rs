fn high_and_low(numbers: &str) -> String {
    let mut max: i32 = i32::MIN;
    let mut min: i32 = i32::MAX;
    
    for elem in numbers.split(' ') {
        let elem32 = elem.parse::<i32>().unwrap();
        if elem32 > max {
            max = elem32;
        }

        if elem32 < min {
            min = elem32;
        }
    }  
    let mut result = String::new();

    result.push_str(max.to_string().as_str());
    result.push(' ');
    result.push_str(min.to_string().as_str());
    
    result
}


// The rusts stdlib doesn't include functions for random number generation. 
// This typically provided by the “rand” external library, we unfortunately 
// don’t have access to external libraries. We get around this by linking to
// rand() in the C standard library.
extern {
  fn rand() -> isize;
}

// Safe wrapper for rand() 
fn crand() -> isize {
  unsafe { rand() as isize }
}

fn crand_range(range: std::ops::Range<isize>) -> isize {
  crand() % (range.end - range.start) + range.start
}

#[test]
fn random_test() {
  for _ in 0..20 {
    let n = (0..crand_range(1..40)).map(|_|{
      crand_range(0..1024) - 512
    }).collect::<Vec<_>>();
    
    let min_max = format!("{} {}",
      n.iter().max().unwrap(),
      n.iter().min().unwrap()
    );
    
    let mut s = String::new();
    for i in n.iter().enumerate() {
      if i.0 != 0 {
        s.push(' ');
      }
      s.push_str(&i.1.to_string()[..])
    }
    
    assert_eq!(min_max, high_and_low(&s[..]));
  }
}

#[test]
fn some_test() {
  assert_eq!("542 -214", high_and_low("4 5 29 54 4 0 -214 542 -64 1 -3 6 -6"));
}

#[test]
fn sort_test() {
  assert_eq!("10 -10", high_and_low("10 2 -2 -10"));
}

#[test]
fn plus_minus_test() {
  assert_eq!("1 -1", high_and_low("1 -1"));
}

#[test]
fn plus_plus_test() {
  assert_eq!("1 1", high_and_low("1 1"));
}

#[test]
fn minus_minus_test() {
  assert_eq!("-1 -1", high_and_low("-1 -1"));
}

#[test]
fn plus_minus_zero_test() {
  assert_eq!("1 -1", high_and_low("1 -1 0"));
}

#[test]
fn plus_plus_zero_test() {
  assert_eq!("1 0", high_and_low("1 1 0"));
}

#[test]
fn minus_minus_zero_test() {
  assert_eq!("1 0", high_and_low("1 1 0"));
}

#[test]
fn single_test() {
  assert_eq!("42 42", high_and_low("42"));
}