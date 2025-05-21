// Prevents additional console window on Windows in release, DO NOT REMOVE!!

use dapis_test_lib::run;

#[tokio::main]
async fn main() {
    run().await;
}
