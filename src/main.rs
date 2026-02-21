mod models;

use models::Severity;
use models::Finding;

fn main() {
    let finding = Finding {
    file_path: String::from("src/main.rs"),
    line_number: 42,
    secret_type: String::from("AWS Access Key"),
    matched_text: String::from("AKIA***hidden***"),
    severity: Severity::High,
};

println!("{:?}", finding);
    }

