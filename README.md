main-server
---

The central server used for managing the databases of the ECE Apprengineering Team at Rowan University.

## backend
Coded in Rust, manages database manipulation using AJAX requests from frontend.

### Dependencies:
* [Rouille 3.0.0](https://github.com/tomaka/rouille)
* [Diesel 1.3.3](https://github.com/diesel-rs/diesel)
* [dotenv 0.13.0](https://github.com/sgrif/rust-dotenv)
* [serde 1.0](https://github.com/serde-rs/serde)
* [serde_json 1.0](https://github.com/serde-rs/json)
* [log 0.4](https://github.com/rust-lang-nursery/log)
* [simplelog](https://github.com/drakulix/simplelog.rs)

## frontend
Coded in HTML/CSS/JS, makes requests to the backend to access database and returns to the user.