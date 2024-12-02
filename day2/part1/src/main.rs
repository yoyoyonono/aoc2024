fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut reports = Vec::<Vec<i32>>::new();
    for line in input.lines() {
        let mut new_report = Vec::new();
        for number in line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
        {
            new_report.push(number)
        }
        reports.push(new_report);
    }
    println!("{:?}", reports);
    let mut num_safe = 0;
    for report in reports {
        // check if all increasing
        let mut all_increasing = true;
        let mut all_decreasing = true;
        let mut smallest_difference = i32::MAX;
        let mut largest_difference = i32::MIN;
        for i in 1..report.len() {
            let difference = report[i] - report[i - 1].abs();
            if report[i] < report[i - 1] {
                all_increasing = false;
            }
            if report[i] > report[i - 1] {
                all_decreasing = false;
            }
            if difference < smallest_difference {
                smallest_difference = difference;
            }
            if difference > largest_difference {
                largest_difference = difference;
            }
        }
        if (all_increasing || all_decreasing)
            && (smallest_difference >= 1 && largest_difference <= 3)
        {
            num_safe += 1;
        }
    }
    println!("{}", num_safe);
}
