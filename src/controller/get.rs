use web_sys::{Request, RequestInit, RequestMode, Response};

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use super::FetchError;

pub async fn get_string(url: &str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = gloo_utils::window();

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().expect("Answer contained no string!"))
}
