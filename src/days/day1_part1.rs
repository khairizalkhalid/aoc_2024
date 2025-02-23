use crate::utils;

trait ContentVecTuple {
    fn calculate_distance_of_two_vectors(self) -> Vec<i32>;
}

impl ContentVecTuple for (Vec<i32>, Vec<i32>) {
    fn calculate_distance_of_two_vectors(self) -> Vec<i32> {
        calculate_distance_of_two_vectors(self.0, self.1)
    }
}

pub fn split_contents_into_two_vectors(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in contents.lines() {
        let items: Vec<&str> = line.split_whitespace().collect();
        if items.len() == 2 {
            if let Ok(item1) = items[0].parse::<i32>() {
                vec1.push(item1);
            }
            if let Ok(item2) = items[1].parse::<i32>() {
                vec2.push(item2);
            }
        }
    }
    vec1.sort();
    vec2.sort();

    (vec1, vec2)
}

// My own implementation to buble sort. Improved sorting should be using the rust default sort.
#[allow(dead_code)]
fn bubble_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let max = vec.len();
    let mut temp: i32;
    let mut swapped: bool;
    for i in 0..max - 1 {
        swapped = false;
        for j in 0..max - i - 1 {
            if vec[j] > vec[j + 1] {
                temp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = temp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    vec
}

fn calculate_distance_of_two_vectors(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect()
}

// my implementation to sum in a vector. Improved version should be using rust iter sum
#[allow(dead_code)]
fn sum_of_vector(vec: Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    let max = vec.len();
    for i in 0..max {
        total += vec[i]
    }
    total
}

pub fn run() {
    //read file
    //seperate into to vector: vec1 and vec2
    //sort each vector
    //calculate distance between vec1 and vec2 and add into vec3
    //calculate sum of vec3
    match utils::file_reader::read_file("day1.txt") {
        Ok(contents) => {
            let total_distance: i32 = split_contents_into_two_vectors(&contents)
                .calculate_distance_of_two_vectors()
                .iter()
                .sum();
            println!("{}", total_distance);
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_contents_into_two_vectors() {
        let contents = "1 2\n3 4\n5 6";
        let (vec1, vec2) = split_contents_into_two_vectors(contents);
        assert_eq!(vec1, vec![1, 3, 5]);
        assert_eq!(vec2, vec![2, 4, 6]);
    }

    #[test]
    fn test_bubble_sort() {
        let unsorted_vec = vec![3, 1, 4, 1, 5, 9];
        let sorted_vec = bubble_sort(unsorted_vec.clone());
        assert_eq!(sorted_vec, vec![1, 1, 3, 4, 5, 9]);
    }

    #[test]
    fn test_calculate_distance_of_two_vectors() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![3, 2, 1];
        let distances = calculate_distance_of_two_vectors(vec1, vec2);
        assert_eq!(distances, vec![2, 0, 2]);
    }

    #[test]
    fn test_sum_of_vector() {
        let vec = vec![1, 2, 3, 4, 5];
        let sum = sum_of_vector(vec);
        assert_eq!(sum, 15);
    }
}
