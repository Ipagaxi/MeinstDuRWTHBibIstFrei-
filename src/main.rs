pub mod http_request;
pub mod parse_json;
pub mod data_prep;

use core::num;

use chrono::{Duration, Utc};
use serde_json::{json, Value};
use serde::Deserialize;
use reqwest::{Error};
use std::{thread, time};

use crate::parse_json::ExamInfo;

const NUM_WEEKDAYS: usize = 16;

#[tokio::main]
async fn main() {
    let mut date = Utc::now().date_naive() - Duration::days(1);
    println!("date: {}", date);
    let mut url= "".to_string();
    let mut exam_info_vec: Vec<ExamInfo> = Vec::new();

    for _ in 0..NUM_WEEKDAYS {
        println!("Fetch exams of {}...", date);
        url = format!("https://online.rwth-aachen.de/RWTHonline/ee/rest/slc.xm.exd/exExamOffer?$filter=exExamDateFrom-gte={};exExamDateTo-lte={}&$orderBy=exExamDate=ascnl&$skip=0&$top=100&orgId=1", date, date);
        match http_request::send_get_request(url.as_str()).await {
            Ok(result) => {
                exam_info_vec.append(&mut parse_json::parse_json(&result));
                date += Duration::days(1);
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
        thread::sleep(time::Duration::from_millis(200));
    }
    println!("Prepare data...");
    data_prep::display_load_for_weekdays(exam_info_vec);
}
