use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlParts {
    pub scheme: String,
    pub host: String,
    pub path: String,
}

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("URL 缺少 scheme")]
    MissingScheme,
    #[error("URL 缺少 host")]
    MissingHost,
}

pub fn parse_url(input: &str) -> Result<UrlParts, UrlError> {
    let (scheme, rest) = input.split_once("://").ok_or(UrlError::MissingScheme)?;
    let (host, path) = match rest.split_once('/') {
        Some((h, p)) => (h.to_string(), format!("/{}", p)),
        None => (rest.to_string(), "/".to_string()),
    };
    if host.is_empty() {
        return Err(UrlError::MissingHost);
    }
    Ok(UrlParts {
        scheme: scheme.to_string(),
        host,
        path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_url() {
        let p = parse_url("https://example.com/docs").unwrap();
        assert_eq!(p.scheme, "https");
        assert_eq!(p.host, "example.com");
        assert_eq!(p.path, "/docs");
    }
}
