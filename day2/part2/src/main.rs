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
        if is_safe(report.clone()) {
            num_safe += 1;
        } else {
            let mut is_any_safe = false;
            for remove_index in 0..report.len() {
                let mut report_clone = report.clone();
                report_clone.remove(remove_index);
                if is_safe(report_clone) {
                    is_any_safe = true;
                }
            }
            if is_any_safe {
                num_safe += 1;
            }
        }
    }
    println!("{}", num_safe);
}

fn is_safe(report: Vec<i32>) -> bool {
    // check if all increasing
    let mut all_increasing = true;
    let mut all_decreasing = true;
    let mut smallest_difference = i32::MAX;
    let mut largest_difference = i32::MIN;
    for i in 1..report.len() {
        if report[i] < report[i - 1] {
            all_increasing = false;
        }
        if report[i] > report[i - 1] {
            all_decreasing = false;
        }
        if (report[i] - report[i - 1]).abs() < smallest_difference {
            smallest_difference = (report[i] - report[i - 1]).abs();
        }
        if (report[i] - report[i - 1]).abs() > largest_difference {
            largest_difference = (report[i] - report[i - 1]).abs();
        }
    }
    return (all_increasing || all_decreasing)
        && (smallest_difference >= 1 && largest_difference <= 3);
}
