use teloxide::types::Message;
use url::Url;
use regex::Regex;
pub fn extract_first_valid_url_from_message(msg: &Message) -> Option<String> {
    let text = msg.text()?;

    let re = Regex::new(r#"((https?://)?[a-zA-Z0-9.-]+\.[a-zA-Z0-9]{2,}(/\S*)?)"#).unwrap();

    for caps in re.captures_iter(text) {
        let url_candidate = caps.get(1)?.as_str();

        let fixed_url = if url_candidate.starts_with("http://") || url_candidate.starts_with("https://") {
            url_candidate.to_string()
        } else {
            format!("http://{}", url_candidate)
        };

        // Проверяем, можно ли распарсить как URL
        if Url::parse(&fixed_url).is_ok() {
            return Some(fixed_url);
        }
    }

    None
}