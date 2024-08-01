# How to run the program?

1. Install Rust on the system
2. `cd` to project root
3. `cargo run`

# How to run the test?

1. `cd` to project root
2. `cargo test`


# API

## /schedule

This API take JSON input with the hours of work that needs to be scheduled and the worker ids that are available for work. It returns the work schedules assigned to each worker


### Request

`{"work": int, "workers":[id,..]}`

- `work` is in integer hours
- `workers` is the id of the available workers as integers

### Response

`[{"worker":id,"start_time":"2020-08-03 00:00:00.0 +00:00:00","end_time":"2020-08-03 08:00:00.0 +00:00:00"},...]`

# Deployment

TODO
