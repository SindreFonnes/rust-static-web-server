use axum::http::HeaderValue;
use serde_json::Value;

const DEFAULT_CSP: &str = concat!(
    "default-src 'none'; ",
    "script-src 'self'; ",
    "script-src-attr 'none'; ",
    "connect-src 'self' wss: ; ",
    "base-uri 'self'; ",
    "font-src 'self' https: data:; ",
    "img-src 'self' data:; ",
    "object-src 'none'; ",
    "style-src 'self' https: 'unsafe-inline'; ",
    "frame-ancestors 'self'; ",
    "form-action 'self'"
);

pub fn get_default_csp() -> HeaderValue {
    HeaderValue::from_static(DEFAULT_CSP)
}

pub fn get_csp_rules(server_config: &Value) -> Option<HeaderValue> {
    let csp_config = server_config.get("CSP")?;

    let csp_string = match csp_config.as_str() {
        Some(csp) => csp,
        None => return None,
    };

    match HeaderValue::from_str(csp_string) {
        Ok(csp) => Some(csp),
        Err(_) => None,
    }
}
