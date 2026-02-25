use crate::{NUM_WEEKDAYS, parse_json};
use chrono::{Utc, Duration};

pub fn display_load_for_weekdays(exam_info_vec: Vec<parse_json::ExamInfo>) {
    let today = Utc::now().date_naive();
    let mut weekdays: [u32; NUM_WEEKDAYS] = [0; NUM_WEEKDAYS];
    let num_weekdays: i64 = (NUM_WEEKDAYS).try_into().unwrap();

    for exam_info in exam_info_vec {
        let exam_day = exam_info.date.date_naive();
        let diff = exam_day - today;
        let diff_usize = diff.num_days() as usize;

        match diff.num_days() {
            -1 => {
                weekdays[0] += exam_info.num_participants;
            },
            0 => {
                weekdays[1] += exam_info.num_participants;
            },
            n => {
                let index = (n+1) as usize;
                weekdays[index] += exam_info.num_participants;
            }
        }
    }

    println!("######################################################");
    println!("Overall number of students writing having an exam:");
    println!("Yesterday ({}): {}", today - Duration::days(1), weekdays[0]);
    println!("Today ({}): {}", today, weekdays[1]);
    for i in 0..num_weekdays-2 {
        let index = (i+2) as usize;
        println!("In {} days: {}", index, weekdays[index])
    }
    println!("######################################################");
}