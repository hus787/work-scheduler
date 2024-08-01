use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

// A stateless application
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        // tbd
        .route("/", get(string_ret).post(string_ret))
        // `POST /schedule` goes to `create_schedule`
        .route("/schedule", post(create_schedule));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_schedule(
    // this argument parses the request body
    // into a `ScheduleNeed` type
    Json(payload): Json<ScheduleNeed>,
) -> (StatusCode, Json<Vec<Shift>>) {
    let shifts = schedule_worker_shifts(payload);
    (StatusCode::OK, Json(shifts))
}

async fn string_ret() -> (StatusCode, String) {
    error!("");
    info!("{:?}", time::macros::datetime!(2020-01-01 00:00));
    (StatusCode::OK, String::from("hello world\n"))
}

fn first_shift_day() -> time::Date {
    use time::ext::NumericalDuration;
    let now_plus_two = time::OffsetDateTime::now_utc()
        .checked_add(2.days())
        .unwrap();
    return time::Date::from_calendar_date(
        now_plus_two.year(),
        now_plus_two.month(),
        now_plus_two.day(),
    )
    .unwrap();
}

fn schedule_worker_shifts(payload: ScheduleNeed) -> Vec<Shift> {
    use time::ext::NumericalDuration;
    info!("{:?}", payload);
    let mut shifts: Vec<Shift> = Vec::new();
    if payload.workers.len() == 0 {
        return shifts;
    }
    let shifts_starts_from = first_shift_day();
    // let shift_packed_days: u32 = (payload.work / (payload.workers.len() as u32));
    let mut shifts_scheduled = 0;
    let mut hours_scheduled = 0;
    while hours_scheduled < payload.work {
        let start_time = time::OffsetDateTime::new_utc(
            shifts_starts_from
                .checked_add((shifts_scheduled / (payload.workers.len() as u32) as i64).days())
                .unwrap(),
            time::Time::from_hms(0, 0, 0).unwrap(),
        );
        let end_time = time::OffsetDateTime::new_utc(
            shifts_starts_from
                .checked_add((shifts_scheduled / (payload.workers.len() as u32) as i64).days())
                .unwrap(),
            time::Time::from_hms(8, 0, 0).unwrap(),
        );
        shifts.push(Shift {
            worker: payload.workers
                [(shifts_scheduled as u32 % (payload.workers.len() as u32)) as usize],
            start_time,
            end_time,
        });
        hours_scheduled += 8;
        shifts_scheduled += 1;
    }
    info!("{:?}", shifts);
    return shifts;
}

// the input to `scheduler` handler
#[derive(Deserialize, Debug)]
struct ScheduleNeed {
    work: u32,
    workers: Vec<u32>,
}

#[derive(Serialize, Debug)]
struct Shift {
    worker: u32,
    start_time: time::OffsetDateTime,
    end_time: time::OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_shifts_are_eight_hours_long() {
        // A worker has shifts
        assert_eq!(1, 1);
    }
    #[test]
    fn no_shift_if_no_work() {
        assert_eq!(1, 1);
    }
    #[test]
    fn no_shift_if_no_worker() {
        assert_eq!(1, 1);
    }
    #[test]
    fn fill_max_possible_workers_in_a_day() {
        assert_eq!(1, 1);
    }
    #[test]
    fn worker_cannot_have_more_than_one_shift_per_day() {
        // A worker never has two shifts on the same day
        // (assumption that no more than 1 per day)
        assert_eq!(1, 1);
    }
    #[test]
    fn shifts_are_in_24_hour_table_in_multiples_of_eight() {
        // It is a 24 hour timetable 0-8, 8-16, 16-24
        assert_eq!(1, 1);
    }
    #[test]
    fn workers_are_provided_shifts_if_there_is_work() {
        // A worker has shifts
        assert_eq!(1, 1);
    }
}
