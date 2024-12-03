use std::fs;
// use std::fmt;
// use std::env;

struct Solution;
impl Solution {
    fn star_one(&self) {
        let file_contents = fs::read_to_string("./input.txt").unwrap();
        let split_contents = file_contents.trim_end().split("\r\n");

        // let mut matrix: Vec<Vec<i32>> = Vec::new();

        let mut safe_reports = 0;

        for line in split_contents {
            // let vec_contents: Vec<&str> = line.split(" ").collect();
            // println!("{:?}", vec_contents);
            let vec_contents: Vec<i32> = line
                .split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            // println!("{:?}", vec_contents);
            // matrix.push(vec_contents);

            if self.is_monotonic(&vec_contents) {
                safe_reports += 1;
                // println!("{:?}", vec_contents);
            }
        }
        // println!("{:?}", matrix);

        println!("{}", safe_reports);
    }

    fn is_monotonic(&self, nums: &Vec<i32>) -> bool {
        let n = nums.len();

        let mut inc = true;
        let mut dec = true;

        for i in 0..n - 1 {
            let diff = nums[i] - nums[i + 1];

            if nums[i] > nums[i + 1] {
                inc = false;
            }
            if nums[i] < nums[i + 1] {
                dec = false;
            }

            if diff.abs() < 1 || diff.abs() > 3 {
                if inc {
                    inc = false;
                }
                if dec {
                    dec = false;
                }
            }
        }

        if inc == false && dec == false {
            return false;
        }

        inc || dec
    }

    // fn decreasing(&self, nums: &Vec<i32>) -> bool {
    //     let n = nums.len();

    //     for i in 0..n - 1 {
    //         if &nums[i] < &nums[i + 1] {
    //             return false;
    //         }
    //     }

    //     true
    // }

    fn star_two(&self) {
        let file_contents = fs::read_to_string("./input.txt").unwrap();
        let split_contents = file_contents.trim_end().split("\r\n");

        let mut safe_reports = 0;

        for line in split_contents {
            let vec_contents: Vec<i32> = line
                .split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            
                if self.is_monotonic(&vec_contents) {
                    safe_reports += 1;
                    // println!("{:?}", vec_contents);
                // if not monotonic, try removing a level
                } else if !self.is_monotonic(&vec_contents){
                    let res = self.remove_level(&vec_contents);
                    if res {
                        safe_reports += 1;
                        // println!("{:?}", vec_contents);
                    }
                }
        }

        println!("{}", safe_reports);
    }

    fn remove_level(&self, vec_contents: &Vec<i32>) -> bool {
        let n = vec_contents.len();

        for i in 0..n {
            let mut vec_clone = vec_contents.clone();
            vec_clone.remove(i);
            if self.is_monotonic(&vec_clone) {
                return true;
            }
        }
        
        false
    }

}

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let s = Solution;
    s.star_one();
    s.star_two();
}
