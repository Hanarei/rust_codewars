fn solution(num: i32) -> i32 {
   let mut num_list = Vec::<i32>::new();
    let mut cur_num = 0;
    while cur_num < num {
        if (cur_num % 5 == 0) || (cur_num % 3 == 0){
            num_list.push(cur_num);
        }
        cur_num += 1;
    }

    num_list.iter().sum()
}

#[test]
fn returns_expected() {
  assert_eq!(solution(10), 23);
  assert_eq!(solution(11), 33);
  assert_eq!(solution(33), 225);
  assert_eq!(solution(6), 8);
  assert_eq!(solution(123), 3420);
  assert_eq!(solution(50), 543);
  assert_eq!(solution(10500), 25719750);
}