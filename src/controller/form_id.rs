use super::{get::get_string, FetchError};

const FORM_ID_URL: &str = "/skyrim_ids.json";

pub async fn fetch_form_ids() -> Result<String, FetchError> {
    get_string(FORM_ID_URL).await
}
