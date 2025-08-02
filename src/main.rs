use std::{env, fmt::Display};

use copypasta::{ClipboardContext, ClipboardProvider};
use regex::Regex;

#[derive(Debug)]
enum Error {
    NotFoundURL,
    FailedParseURL,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFoundURL => write!(f, "not found url. plsease set url as args 1"),
            Error::FailedParseURL => write!(f, "failed parse youtube url"),
        }
    }
}

/// https://www.youtube.com/watch?v=`video_id` -> https://youtu.be/`video_id?si=share_id
fn remove_from_youtube(url: &str) -> Result<String, Error> {
    let re = Regex::new(r"watch\?v=([^&]*)").unwrap();

    let v_id = match re.captures(url) {
        Some(v) => Ok(v[1].to_string()),
        None => Err(Error::FailedParseURL),
    }?;

    Ok(format!("https://youtu.be/{v_id}"))
}

fn generic_remover(url: &str) -> Result<String, Error> {
    let re = Regex::new(r"^(.*?)\?").unwrap();

    Ok(re
        .captures(url)
        .map(|f| f[1].to_string())
        .unwrap_or(url.to_string()))
}

fn main() -> Result<(), Error> {
    let args = env::args().collect::<Vec<String>>();
    let url = args.get(1).ok_or(Error::NotFoundURL)?;
    let res = if url.contains("youtube") {
        remove_from_youtube(url)
    } else {
        generic_remover(url)
    }?;

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(res.clone()).unwrap();

    println!("pasted to clipboard: {res}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_from_youtube() {
        let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=42s";
        let expected = "https://youtu.be/dQw4w9WgXcQ";
        assert_eq!(remove_from_youtube(url).unwrap(), expected);
    }

    #[test]
    fn test_remove_from_youtube_no_v() {
        let url = "https://www.youtube.com/watch?t=42s";
        assert!(remove_from_youtube(url).is_err());
    }

    #[test]
    fn test_generic_remover() {
        let url = "https://example.com/page?param1=value1&param2=value2";
        let expected = "https://example.com/page";
        assert_eq!(generic_remover(url).unwrap(), expected);
    }

    #[test]
    fn test_generic_remover_no_query() {
        let url = "https://example.com/page";
        assert_eq!(generic_remover(url).unwrap(), url);
    }
}
