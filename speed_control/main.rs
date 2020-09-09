fn gps(s: i32, x: Vec<f64>) -> i32 {
    if x.is_empty() || x.len() == 1{
        return 0;
    }
    
    let _speed: f64   = s as f64;
    let mut result    = Vec::<i32>::new();
    let mut prev: f64 = x[0];

    for item in x.iter().skip(1){
        let res = (item - prev) * 3600.0 / _speed;
        result.push(res as i32);
        prev = *item;
    }
    
    *result.iter().max_by(|x, y| x.cmp(y)).unwrap()
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(s: i32, x: Vec<f64>, exp: i32) -> () {
        println!("s: {:?};", s);
        println!("x: {:?};", x);
        let ans = gps(s, x);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut x = vec![0.0, 0.23, 0.46, 0.69, 0.92, 1.15, 1.38, 1.61];
        let mut s = 20;
        let mut u = 41;
        dotest(s, x, u);
        x = vec![0.0, 0.11, 0.22, 0.33, 0.44, 0.65, 1.08, 1.26, 1.68, 1.89, 2.1, 2.31, 2.52, 3.25];
        s = 12;
        u = 219;
        dotest(s, x, u);
        x = vec![0.0, 0.18, 0.36, 0.54, 0.72, 1.05, 1.26, 1.47, 1.92, 2.16, 2.4, 2.64, 2.88, 3.12, 3.36, 3.6, 3.84];
        s = 20;
        u = 80;
        dotest(s, x, u);
        x = vec![0.0, 0.01, 0.36, 0.6, 0.84, 1.05, 1.26, 1.47, 1.68, 1.89, 2.1, 2.31, 2.52, 2.73, 2.94, 3.15];
        s = 14;
        u = 90;
        dotest(s, x, u);
        x = vec![0.0, 0.02, 0.36, 0.54, 0.72, 0.9, 1.08, 1.26, 1.44, 1.62, 1.8];
        s = 17;
        u = 72;
        dotest(s, x, u);
        x = vec![0.0, 0.24, 0.48, 0.72, 0.96, 1.2, 1.44, 1.68, 1.92, 2.16, 2.4];
        s = 12;
        u = 72;
        dotest(s, x, u);
        x = vec![0.0, 0.02, 0.44, 0.66, 0.88, 1.1, 1.32, 1.54, 1.76];
        s = 17;
        u = 88;
        dotest(s, x, u);
        x = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.32, 1.54, 1.76, 1.98, 2.2, 2.42, 2.76, 2.99, 3.22, 3.45];
        s = 16;
        u = 76;
        dotest(s, x, u);
        x = vec![0.0, 0.01, 0.36, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0, 3.25, 3.5, 3.75, 4.0, 4.25, 4.5, 4.75];
        s = 17;
        u = 82;
        dotest(s, x, u);
        x = vec![0.0, 0.2, 0.4, 0.69, 0.92, 1.15, 1.38, 1.61, 1.92, 2.16, 2.4, 2.64, 2.88, 3.12, 3.36];
        s = 19;
        u = 58;
        dotest(s, x, u);
        x = vec![];
        s = 19;
        u = 0;
        dotest(s, x, u);
        x = vec![0.0];
        s = 19;
        u = 0;
        dotest(s, x, u);
    }

    extern crate rand;
    use self::rand::Rng;
        
    fn do_array(k: i32) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let mut r: Vec<f64> = vec![];
        let mut prev = 0.0;
        let mut i = 0;
        while i < k {
            let mut v = rng.gen_range(2, 4) as f64;
            while v <= prev {
                v = v * (rng.gen_range(2, 5) as f64);
            }
            if v > 800.0 {break;}
            if rng.gen_range(0, 100) % 5 == 0 {
                v = prev;
            } else {
                prev = v;
            }
            r.push(v / 100.0);
            i += 1;
        }
        return r;
    }
    fn gps_cli(s: i32, x: Vec<f64>) -> i32 {
        if x.len() < 2 {return 0;}
        let mut max = x[1] - x[0];
        for i in 2..x.len() {
            let v = x[i] - x[i-1];
            if v > max {
                max = v;
            }
        }
        return (max * 3600.0 / (s as f64)).floor() as i32
    }

    #[test]
    fn random_tests() {
        let mut rng = rand::thread_rng();
        for _ in 0..200 {
            let s = rng.gen_range(35, 60);
            let x = do_array(rng.gen_range(10, 16));
            let y = x.clone();
            let sol = gps_cli(s, x);
            dotest(s, y, sol);
        }
    }
}