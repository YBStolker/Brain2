#[get("/")]
pub fn index() -> &'static str {
    "Shared input"
}