fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = arr.to_vec();
    res.sort();
    res
    
}

#[cfg(test)]
extern crate rand;


#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn solution(arr: &Vec<i32>) -> Vec<i32> {
        let mut clone = arr.clone();
        clone.sort();

        return clone;
    }

    #[test]
    fn sample_tests() {
        assert_eq!(sort_numbers(&vec![1, 2, 3, 10, 5]), vec![1, 2, 3, 5, 10]);
        assert_eq!(sort_numbers(&vec![]), vec![]);
        assert_eq!(sort_numbers(&vec![20, 2, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 20, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 10, 20]), vec![2, 10, 20]);
    }

    #[test]
    fn random_tests() {
        for _i in 0..100 {
            // Makes a random vector with length (rnd 0 to 100) with elements (rnd 0 to 100)
            let arr : Vec<i32> = (0..rand::thread_rng().gen_range(0, 101)).map(|_| rand::thread_rng().gen_range(0, 101)).collect();

            assert_eq!(sort_numbers(&arr), solution(&arr));
        }
    }
}