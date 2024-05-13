use std::collections::HashMap;

fn main() {
    // Sum of Even Numbers:
    assert_eq!(
        36,
        sum_of_even_numbers(vec![1, 2, 3, 5, 6, 7, 8, 9, 0, 1, 2, 4, 6, 8, 9])
    );
    assert_eq!(0, sum_of_even_numbers(vec![1, 3, 5, 7, 9, 0, 1, 9]));

    // Reverse String:
    assert_eq!(
        String::from("neevan"),
        reverse_string(String::from("Naveen"))
    );
    assert_eq!(
        String::from("asdfghjkl"),
        reverse_string(String::from("lkjhgfDSA"))
    );

    // Fibonacci Sequence:
    assert_eq!(vec![0, 1, 1, 2, 3, 5, 8], fibonacci_sequence(7));
    assert_eq!(vec![0, 1, 1, 2, 3], fibonacci_sequence(5));
    assert_eq!(vec![0], fibonacci_sequence(1));
    let emp_vec: Vec<i32> = Vec::new();
    assert_eq!(emp_vec, fibonacci_sequence(0));
    assert_eq!(emp_vec, fibonacci_sequence(-10));

    // Factorial Calculation:
    assert_eq!(3628800, factorial_calculation(10));
    assert_eq!(1, factorial_calculation(0));

    // Palindrome Check:
    assert_eq!(true, palindrome_check("asdfghgfdsa"));
    assert_eq!(false, palindrome_check("naveen"));

    // Prime Number Check:
    assert_eq!(false, prime_number_check(10));
    assert_eq!(true, prime_number_check(7));

    // String Concatenation:
    assert_eq!("hello world", string_concantation("hello", "world"));
    assert_eq!("I am naveen", string_concantation("I am", "naveen"));

    // Count Occurrences:
    assert_eq!(
        3,
        count_occurance_for_single_digit(vec![1, 2, 3, 4, 4, 3, 3], 3)
    );
    assert_eq!(
        0,
        count_occurance_for_single_digit(vec![1, 2, 3, 4, 4, 3, 3], 13)
    );
    let mut test_hm: HashMap<i32, i32> = HashMap::new();
    test_hm.insert(6, 1);
    test_hm.insert(1, 1);
    test_hm.insert(8, 2);
    test_hm.insert(2, 2);
    test_hm.insert(4, 2);
    test_hm.insert(5, 1);
    test_hm.insert(7, 1);
    test_hm.insert(3, 2);
    assert_eq!(
        test_hm,
        count_occurance_of_all_elements(vec![1, 2, 3, 4, 3, 2, 8, 4, 5, 6, 7, 8])
    );

    // Remove Duplicates:
    assert_eq!(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        remove_duplicate_numbers(vec![1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 3, 2, 1])
    );
    assert_eq!(emp_vec, remove_duplicate_numbers(vec![]));

    // Check Anagrams:
    assert_eq!(true, check_anagram("silent", "listen"));
    assert_eq!(false, check_anagram("silentq", "listen"));
    assert_eq!(false, check_anagram("silent", "abcdef"));
}

fn sum_of_even_numbers(array: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in array.iter() {
        if i % 2 == 0 {
            sum += i;
        }
    }
    sum
}

fn reverse_string(s: String) -> String {
    let mut new_str = String::from("");
    for i in s.chars().into_iter() {
        new_str.insert(0, i.to_ascii_lowercase());
    }
    new_str
}

fn fibonacci_sequence(n: i32) -> Vec<i32> {
    if n <= 0 {
        return vec![];
    }
    let mut seq: Vec<i32> = vec![0, 1];
    if n == 1 {
        vec![0]
    } else if n == 2 {
        vec![0, 1]
    } else {
        for _i in 3..=n {
            let next = seq[seq.len() - 1] + seq[seq.len() - 2];
            seq.push(next);
        }
        seq
    }
}

fn factorial_calculation(n: i32) -> i32 {
    let mut sum = 1;
    for i in 2..=n {
        sum *= i
    }
    sum
}

fn palindrome_check(s: &str) -> bool {
    for i in 0..=s.len() / 2 {
        if s.chars().nth(i) != s.chars().nth(s.len() - i - 1) {
            return false;
        }
    }
    true
}

fn prime_number_check(n: i32) -> bool {
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn string_concantation(s1: &str, s2: &str) -> String {
    let mut concat: String = s1.to_string();
    concat.push(' ');
    concat.push_str(s2);
    concat
}

fn count_occurance_for_single_digit(arr: Vec<i32>, target: i32) -> i32 {
    let mut count = 0;
    for i in arr.iter() {
        if *i == target {
            count += 1;
        }
    }
    count
}

fn count_occurance_of_all_elements(arr: Vec<i32>) -> HashMap<i32, i32> {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for i in arr.iter() {
        match hm.get(i) {
            Some(n) => {
                hm.insert(*i, n + 1);
            }
            _ => {
                hm.insert(*i, 1);
            }
        }
    }
    hm
}

fn remove_duplicate_numbers(arr: Vec<i32>) -> Vec<i32> {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    let mut ans_arr: Vec<i32> = Vec::new();
    for i in arr.iter() {
        match hm.get(i) {
            Some(_) => {
                continue;
            }
            None => {
                hm.insert(*i, 1);
                ans_arr.push(*i);
            }
        }
    }
    ans_arr
}

fn check_anagram(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut hm: HashMap<char, i32> = HashMap::new();
    for i in s1.chars() {
        match hm.get(&i) {
            Some(n) => {
                hm.insert(i, n + 1);
            }
            _ => {
                hm.insert(i, 1);
            }
        }
    }
    for i in s2.chars() {
        match hm.get(&i) {
            Some(n) => {
                if n == &1 {
                    hm.remove(&i);
                } else {
                    hm.insert(i, n - 1);
                }
            }
            _ => {
                return false;
            }
        }
    }
    if hm.len() == 0 {
        return true;
    }
    false
}
