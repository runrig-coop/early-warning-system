use chrono::NaiveDate;
use serde;

#[derive(serde::Serialize)]
pub struct Farm {
    pub id: usize,
    pub name: String,
    // pub measurements: Vec<Measurement>,
    pub status: Status,
    pub timestamp: usize,
}
  
pub struct Planting {
    pub id: usize,
    pub name: String,
    pub crop: String,
    pub land: usize,
    pub last_on_site: NaiveDate,
    pub drive_time: f32,
    pub start_date: NaiveDate,
    pub expected_end: NaiveDate,
    pub actual_end: NaiveDate,
    pub measurements: Vec<Measurement>
}

pub struct Land {
    pub id: usize,
    pub farm: usize,
    pub lat: f32,
    pub long: f32,
    pub address: String,
    pub measurements: Vec<Measurement>
}

pub enum Measurement {
    Quantitative(Quantitative),  
    Qualitative(Qualitative), 
}
  
pub struct Quantitative {
    pub id: usize,
    pub kind: String,
    pub value: f32,
    pub date: NaiveDate,
}
  
pub struct Qualitative {
    pub id: usize,
    pub kind: String,
    pub value: String,
    pub note: String,
    pub date: NaiveDate,
}

#[derive(serde::Serialize)]
pub enum Status {
    Red,
    Yellow,
    Green
}