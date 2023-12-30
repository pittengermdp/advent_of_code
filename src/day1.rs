pub const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aoc(day1, part1)]
#[must_use]
pub fn part1(input: &str) -> i32 {
    //For each line
    //  we need to get the first numeric and the last numeric and concatenate them
    //  in order to make a single u32.
    //Then sum them together
    input
        .lines()
        .map(|line| {
            let first_num = char::to_digit(
                line.chars().find(|x| char::is_numeric(*x)).unwrap_or('0'),
                10,
            )
            .unwrap_or_default()
                * 10;
            let last_num = char::to_digit(
                line.chars()
                    .rev()
                    .find(|x| char::is_numeric(*x))
                    .unwrap_or('0'),
                10,
            )
            .unwrap_or_default();
            first_num + last_num
        })
        .sum::<u32>()
        .try_into()
        .unwrap_or_default()
}

#[aoc(day1, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    //For each line
    // For the first number look at each character, if that character is a assign that digit to first_num.
    //  Otherwise, look at that character through the end of the line and see if it starts with one of our words.

    //Do the same but starting from the end of the line and working backwards.
    // If the character is a digit, assign that digit to second_num.
    //  Otherwise, look at that character through to the end of the line and see if it starts with one our our words.
    input
        .lines()
        .map(|line| {
            let mut first_num = None;
            let mut second_num = None;

            for (line_idx, c) in line.chars().enumerate() {
                first_num = char::to_digit(c, 10).map_or_else(
                    || {
                        let mut j = 0;
                        for word in &NUMBER_WORDS {
                            if line[line_idx..].starts_with(word) {
                                return Some(j + 1);
                            }
                            j += 1;
                        }
                        None
                    },
                    |num| Some(num as usize),
                );

                if first_num.is_some() {
                    break;
                }
            }
            for (line_idx, c) in line.chars().rev().enumerate() {
                second_num = char::to_digit(c, 10).map_or_else(
                    || {
                        let mut j = 0;
                        for word in &NUMBER_WORDS {
                            if line[line.len() - line_idx - 1..].starts_with(word) {
                                return Some(j + 1);
                            } else {
                                j += 1;
                            };
                        }
                        None
                    },
                    |num| Some(num as usize),
                );
                if second_num.is_some() {
                    break;
                }
            }
            first_num.unwrap_or_default() * 10 + second_num.unwrap_or_default()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_no_ints_test() {
        let input = "abcde";
        let expected = 0;
        let actual = part1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_1_one_int_test() {
        let input = "a1cde";
        let expected = 11;
        let actual = part1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_1_multiple_ints_test() {
        let input = "a1c32e";
        let expected = 12;
        let actual = part1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_1_multiple_lines() {
        let input = "a1c32e\nasdfawer\na1c36e";
        let expected = 28;
        let actual = part1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_1_solution() {
        let input = include_str!("../input/2023/day1.txt");
        let expected = 54708;
        let actual = part1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_2_no_ints_test() {
        let input = "abcde";
        let expected = 0;
        let actual = part2(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_2_only_word_test() {
        let input = "one";
        let expected = 11;
        let actual = part2(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_2_only_word_and_digit_test() {
        let input = "2asdfasdone";
        let expected = 21;
        let actual = part2(input);
        assert_eq!(expected, actual);

        let input = "nineasdfasd3";
        let expected = 93;
        let actual = part2(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_2_digit_in_middle_of_word_test() {
        let input = "2asdfasdone";
        let expected = 21;
        let actual = part2(input);
        assert_eq!(expected, actual);

        let input = "n3ineasdfasd3";
        let expected = 33;
        let actual = part2(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_2_solution() {
        let input = include_str!("../input/2023/day1.txt");
        let expected = 54087;
        let actual = part2(input);
        assert_eq!(expected, actual);
    }
}
