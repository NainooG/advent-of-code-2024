use std::{collections::HashMap, fs};

struct Solution;
impl Solution {
    fn star_one(&self) {
        let file_contents = fs::read_to_string("./input.txt").unwrap();
        let split_contents = file_contents.trim_end().split("\r\n");

        let mut total_distance = 0;

        let mut vec1: Vec<u32> = Vec::new();
        let mut vec2: Vec<u32> = Vec::new();

        for line in split_contents {
            let res: Vec<&str> = line.split("   ").collect();
            let num1: u32 = res[0].parse().unwrap();
            let num2: u32 = res[1].parse().unwrap();

            vec1.push(num1);
            vec2.push(num2);
        }

        vec1.sort();
        vec2.sort();

        for idx in 0..vec1.len() {
            let distance = &vec1[idx].abs_diff(vec2[idx]);
            total_distance += distance;
        }

        println!("{}", total_distance);
    }

    fn star_two(&self) {
        let file_contents = fs::read_to_string("./input.txt").unwrap();
        let split_contents = file_contents.trim_end().split("\r\n");

        let mut similarity_score = 0;

        let mut vec1: Vec<u32> = Vec::new();
        let mut vec2: Vec<u32> = Vec::new();

        for line in split_contents {
            let res: Vec<&str> = line.split("   ").collect();
            let num1: u32 = res[0].parse().unwrap();
            let num2: u32 = res[1].parse().unwrap();

            vec1.push(num1);
            vec2.push(num2);
        }

        let mut hash: HashMap<u32, u32> = HashMap::new();

        for n in vec2.iter() {
            let entry = hash.entry(*n).or_insert(0);
            *entry += 1;
        }

        for n in vec1.iter() {
            let current_similarity = n * hash.get(n).copied().unwrap_or(0);
            similarity_score += current_similarity;
        }
        
        println!("{}", similarity_score);

    }
}

fn main() {
    let s = Solution;
    s.star_one();
    s.star_two();
}
