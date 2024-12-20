use crate::file_handling;

pub fn first_puzzle() -> i32 {
    let reports = parse_reports();

    let mut safe_reports = 0;
    for report in reports {
        if is_report_safe(&report) {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn second_puzzle() -> i32 {
    let reports = parse_reports();

    let mut safe_reports = 0;
    for report in reports {
        if is_report_safe(&report) {
            safe_reports += 1;
        } else {
            for i in 0..report.len() {
               let sub_report =  report
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, val)| if idx != i { Some(*val) } else { None })
                        .collect();
                if is_report_safe(&sub_report) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    safe_reports
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    // For marking if the Report is asc or desc
    let mut direction = 0;

    for i in 0..report.len() - 1 {
        match report[i] - report[i + 1] {
            0 => return false,
            -3..=-1 => {
                if direction == 0 {
                    direction = -1
                } else if direction == 1 {
                    return false;
                }
            }
            1..=3 => {
                if direction == 0 {
                    direction = 1
                } else if direction == -1 {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

fn parse_reports() -> Vec<Vec<i32>> {
    let input_content = file_handling::read_day_input(2);

    let mut reports: Vec<Vec<i32>> = vec![];
    for line in input_content.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(report);
    }
    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_puzzle() {
        assert_eq!(first_puzzle(), 242)
    }

    #[test]
    fn test_second_puzzle() {
        assert_eq!(second_puzzle(), 311)
    }
}
