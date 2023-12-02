mod day01 {
    fn find_first_digit(input: &String) -> u32 {
        let result = input
            .find(|c: char| c.is_digit(10))
            .map(|i: usize| input.chars().nth(i).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap_or(0);
        // println!("first={v}", v = result);
        result
    }

    fn find_last_digit(input: &String) -> u32 {
        let result = input
            .rfind(|c: char| c.is_digit(10))
            .map(|i: usize| input.chars().nth(i).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap_or(0);
        // println!("last={v}", v = result);
        result
    }

    fn resolve_star1(lines: Vec<String>) -> i32 {
        lines
            .iter()
            .map(|line| (find_first_digit(line) * 10 + find_last_digit(line)) as i32)
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use std::env;
        use std::fs;

        fn read_lines(filename: &str) -> Vec<String> {
            fs::read_to_string(filename)
                .unwrap()
                .lines()
                .map(String::from)
                .collect()
        }

        #[test]
        fn result_star1_test() {
            let data1 = read_lines("data/day01/example-1.txt");
            let result1 = super::resolve_star1(data1);
            assert_eq!(result1, 142);

            let data2 = read_lines("data/day01/puzzle-1.txt");
            let result2 = super::resolve_star1(data2);
            assert_eq!(result2, 55172);
        }
    }
}
