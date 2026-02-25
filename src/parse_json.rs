use chrono::{DateTime, NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    #[serde(rename = "examOffers")]
    exam_offers: Option<Vec<ExamOffer>>,
}

#[derive(Debug, Deserialize)]
struct LangValue {
    value: String,
}

#[derive(Debug, Deserialize)]
struct DateValue {
    value: String,
}

#[derive(Debug, Deserialize)]
struct ExamOffer {
    #[serde(rename = "courseName")]
    course_name: Option<LangValue>,

    #[serde(rename = "numberOfParticipants")]
    number_of_participants: Option<u32>,

    #[serde(rename = "examDate")]
    exam_date: Option<DateValue>,
}

#[derive(Debug)]
pub struct ExamInfo {
    pub course_name: String,
    pub num_participants: u32,
    pub date: DateTime<Utc>,
}

pub fn parse_json(json_str: &str) -> Vec<ExamInfo> {
    let response: ApiResponse =
        serde_json::from_str(json_str).expect("Invalid JSON");

    match response.exam_offers {
        Some(exam_offers) => {
            exam_offers
            .into_iter()
            .filter_map(|exam| {
                let course = exam.course_name?;
                let participants = exam.number_of_participants?;
                let date_str = exam.exam_date?;

                let naive_date =
                    NaiveDate::parse_from_str(&date_str.value, "%Y-%m-%d").ok()?;

                let datetime =
                    DateTime::<Utc>::from_naive_utc_and_offset(
                        naive_date.and_hms_opt(0, 0, 0)?,
                        Utc,
                    );

                Some(ExamInfo {
                    course_name: course.value,
                    num_participants: participants,
                    date: datetime,
                })
            })
            .collect()
        },
        None => {
            println!("No values found");
            Vec::new()
        }
    }
    
}