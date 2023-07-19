// Looks into the folder that has the same name (`others`) at the same level.
// There it can find two files: `answer.rs` and `another_answer.rs`
// Then export both by using `pub mod <filename without extension>`
pub mod answer;
pub mod another_answer;
