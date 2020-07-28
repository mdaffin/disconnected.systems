use err_derive::Error;
use serde;
use serde::de::DeserializeOwned;
use serde_json::{self, Deserializer};
use serde_yaml;
use toml;

/// Extracts the frontmatter from the given content. It supports three types of frontmatter, YAML
/// TOML and JSON. YAML frontmatter must start and end with the line `---`, TOML with the line
/// `+++` and JSON must start with a valid json object (ie between brackets `{ ... }`).
///
/// The given reader is advanced to the point where the content starts. Continue reading from it to
/// get the content without the frontmatter.
///
/// For example a YAML frontmatter:
/// ```yaml
/// ---
/// title: some title
/// slug: some-url
/// ---
/// post content
/// ```
///
/// TOML frontmatter:
/// ```toml
/// +++
/// title = "some title"
/// slug = "some-url"
/// +++
/// post content
/// ```
///
/// and JSON frontmatter:
/// ```json
/// {
///   "title": "some title",
///   "slug": "some-url"
/// }
/// post content
/// ```
pub fn parse<T, R>(reader: &mut R) -> Result<Option<T>, FrontmatterError>
where
    T: DeserializeOwned,
    R: std::io::BufRead + std::io::Seek,
{
    let mut first_line = String::with_capacity(4);
    while first_line.trim() == "" {
        if reader.read_line(&mut first_line)? == 0 {
            break;
        }
    }
    match first_line.as_str() {
        "---\n" => parse_func(reader, "---", |content| {
            serde_yaml::from_str(content).map_err(FrontmatterError::YAMLError)
        }),
        "+++\n" => parse_func(reader, "+++", |content| {
            toml::from_str(content).map_err(FrontmatterError::TOMLError)
        }),
        line if line.starts_with("{") => {
            reader.seek(std::io::SeekFrom::Start(0))?;
            let frontmatter = parse_json(reader)?;
            // skip the next new line, there is liekly a better way to do this
            reader.seek(std::io::SeekFrom::Current(1))?;
            Ok(frontmatter)
        }
        _ => {
            reader.seek(std::io::SeekFrom::Start(0))?;
            Ok(None)
        }
    }
}

fn parse_func<T, R, F>(reader: &mut R, sep: &str, parser: F) -> Result<Option<T>, FrontmatterError>
where
    T: DeserializeOwned,
    R: std::io::BufRead + std::io::Seek,
    F: FnOnce(&str) -> Result<T, FrontmatterError>,
{
    let mut buf = String::with_capacity(1024);
    while !buf.trim_end().ends_with(sep) {
        if reader.read_line(&mut buf)? == 0 {
            break;
        }
    }

    let frontmatter = buf.trim_end().trim_end_matches(sep);
    Ok(if frontmatter.trim() != "" {
        Some(parser(frontmatter)?)
    } else {
        None
    })
}

fn parse_json<T, R>(reader: &mut R) -> Result<Option<T>, FrontmatterError>
where
    T: DeserializeOwned,
    R: std::io::BufRead + std::io::Seek,
{
    let frontmatter = Deserializer::from_reader(reader)
        .into_iter::<T>()
        .next()
        .unwrap()?;
    Ok(Some(frontmatter))
}

#[derive(Debug, Error)]
/// Errors that can occur during parsing of the pages frontmatter.
pub enum FrontmatterError {
    #[error(display = "{}", _0)]
    Io(#[error(source)] std::io::Error),
    #[error(display = "{}", _0)]
    JSONError(#[error(source)] serde_json::Error),
    #[error(display = "{}", _0)]
    YAMLError(#[error(source)] serde_yaml::Error),
    #[error(display = "{}", _0)]
    TOMLError(#[error(source)] toml::de::Error),
}

#[cfg(test)]
mod tests {
    use super::{parse, FrontmatterError};
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    use serde_json;
    use serde_yaml;
    use std::io::{Cursor, Read};
    use toml;

    fn parse_content<T: DeserializeOwned>(
        content: &str,
    ) -> Result<(Option<T>, String), FrontmatterError> {
        let mut reader = Cursor::new(content);
        let frontmatter: Option<T> = parse(&mut reader)?;
        let mut body = String::new();
        reader.read_to_string(&mut body)?;

        Ok((frontmatter, body))
    }

    #[test]
    /// Frontmatter is optional, if it is missing then the file should be returned unaltered.
    fn missing_frontmatter_returns_whole_file() {
        let content = "this is a file";

        let (frontmatter, body) = parse_content::<serde_json::Value>(&content).unwrap();

        assert_eq!(frontmatter, None);
        assert_eq!(body, content);
    }

    #[test]
    /// If yaml or toml frontmatter is present (ie the content starts with `---\n` or `+++\n` or a
    /// json object) it should not be included in the returned body.
    fn returned_body_does_not_contain_frontmatter() {
        for original_frontmatter in ["---\n---", "+++\n+++", "{}", "{\n}"].iter() {
            let original_body = "this is a file";
            let content = format!("{}\n{}", original_frontmatter, original_body);
            println!("#### Content ####\n{}\n#################", &content);

            let (_, body) = parse_content::<serde_json::Value>(&content).unwrap();

            assert_eq!(body, original_body);
        }
    }

    #[test]
    /// Frontmatter should parse into a given struct.
    fn parses_frontmatter() {
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        struct TestFrontMatter {
            title: String,
        }

        let original_frontmatter = TestFrontMatter {
            title: "test".into(),
        };

        let test_cases = [
            format!(
                "{}\n---",
                serde_yaml::to_string(&original_frontmatter).unwrap()
            ),
            format!(
                "+++\n{}+++",
                toml::to_string(&original_frontmatter).unwrap(),
            ),
            serde_json::to_string(&original_frontmatter).unwrap(),
            serde_json::to_string_pretty(&original_frontmatter).unwrap(),
        ];

        for serialized_frontmatter in test_cases.iter() {
            let original_body = "this is a file";
            let content = format!("{}\n{}", serialized_frontmatter, original_body);
            println!("Content:\n{}\n", &content);

            let (frontmatter, _) = parse_content::<TestFrontMatter>(&content).unwrap();
            assert_eq!(frontmatter, Some(original_frontmatter.clone()));
        }
    }

    #[test]
    /// Frontmatter should parse vectors in the frontmatter
    fn parses_vectors_in_struct() {
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        struct TestFrontMatter {
            title: String,
            vec: Vec<i64>,
        }

        let original_frontmatter = TestFrontMatter {
            title: "test".into(),
            vec: vec![4, 6, 2, -66],
        };

        let test_cases = [
            format!(
                "{}\n---",
                serde_yaml::to_string(&original_frontmatter).unwrap()
            ),
            format!(
                "+++\n{}+++",
                toml::to_string(&original_frontmatter).unwrap(),
            ),
            serde_json::to_string(&original_frontmatter).unwrap(),
            serde_json::to_string_pretty(&original_frontmatter).unwrap(),
        ];

        for serialized_frontmatter in test_cases.iter() {
            let original_body = "this is a file";
            let content = format!("{}\n{}", serialized_frontmatter, original_body);

            println!("Content:\n{}\n", &content);

            let (frontmatter, _) = parse_content::<TestFrontMatter>(&content).unwrap();

            assert_eq!(frontmatter, Some(original_frontmatter.clone()));
        }
    }

    #[test]
    /// Frontmatter should parse nested structs
    fn parses_nested_structs() {
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        struct Inner {
            value: String,
        }

        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        struct TestFrontMatter {
            inner: Inner,
        }

        let original_frontmatter = TestFrontMatter {
            inner: Inner {
                value: "Some value".into(),
            },
        };

        let test_cases = [
            format!(
                "{}\n---",
                serde_yaml::to_string(&original_frontmatter).unwrap()
            ),
            format!(
                "+++\n{}+++",
                toml::to_string(&original_frontmatter).unwrap(),
            ),
            serde_json::to_string(&original_frontmatter).unwrap(),
            serde_json::to_string_pretty(&original_frontmatter).unwrap(),
        ];

        for serialized_frontmatter in test_cases.iter() {
            let original_body = "this is a file";
            let content = format!("{}\n{}", serialized_frontmatter, original_body);

            println!("Content:\n{}\n", &content);

            let (frontmatter, body) = parse_content::<TestFrontMatter>(&content).unwrap();

            assert_eq!(body, original_body);
            assert_eq!(frontmatter, Some(original_frontmatter.clone()));
        }
    }

    #[test]
    /// Only the first frontmatter should be parsed, everything after the first frontmatter block
    /// should be part of the body.
    fn parses_frontmatter_only_once() {
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        struct TestFrontMatter {
            title: String,
        }

        let original_frontmatter = TestFrontMatter {
            title: "test".into(),
        };
        let extra_frontmatter = TestFrontMatter {
            title: "extra".into(),
        };

        let test_cases = [
            (
                format!(
                    "{}\n---",
                    serde_yaml::to_string(&original_frontmatter).unwrap(),
                ),
                format!(
                    "{}\n---",
                    serde_yaml::to_string(&extra_frontmatter).unwrap(),
                ),
            ),
            (
                format!(
                    "+++\n{}+++",
                    toml::to_string(&original_frontmatter).unwrap(),
                ),
                format!("{}+++", toml::to_string(&extra_frontmatter).unwrap(),),
            ),
            (
                format!("{}", serde_json::to_string(&original_frontmatter).unwrap(),),
                format!("\n{}", serde_json::to_string(&extra_frontmatter).unwrap(),),
            ),
            (
                format!("{}", serde_json::to_string(&original_frontmatter).unwrap(),),
                format!("{}", serde_json::to_string(&extra_frontmatter).unwrap(),),
            ),
            (
                format!(
                    "{}",
                    serde_json::to_string_pretty(&original_frontmatter).unwrap(),
                ),
                format!("{}", serde_json::to_string(&extra_frontmatter).unwrap(),),
            ),
            (
                format!("{}", serde_json::to_string(&original_frontmatter).unwrap(),),
                format!(
                    "{}",
                    serde_json::to_string_pretty(&extra_frontmatter).unwrap(),
                ),
            ),
            (
                format!(
                    "{}",
                    serde_json::to_string_pretty(&original_frontmatter).unwrap(),
                ),
                format!(
                    "{}",
                    serde_json::to_string_pretty(&extra_frontmatter).unwrap(),
                ),
            ),
        ];

        for (first, second) in test_cases.iter() {
            let original_body = format!("{}\nthis is a file", second);
            let content = format!("{}\n{}", first, original_body);

            println!("Content:\n{}\n", &content);

            let (frontmatter, body) = parse_content::<TestFrontMatter>(&content).unwrap();

            assert_eq!(body, original_body);
            assert_eq!(frontmatter, Some(original_frontmatter.clone()));
        }
    }

    #[test]
    /// Having no content after the frontmatter should produce an empty body.
    fn no_content_after_frontmatter_is_not_an_error() {
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        struct TestFrontMatter {
            i: u32,
        }

        let original_frontmatter = TestFrontMatter { i: 1 };

        let test_cases = [
            format!(
                "{}\n---",
                serde_yaml::to_string(&original_frontmatter).unwrap()
            ),
            format!(
                "+++\n{}+++",
                toml::to_string(&original_frontmatter).unwrap(),
            ),
            serde_json::to_string(&original_frontmatter).unwrap(),
            serde_json::to_string_pretty(&original_frontmatter).unwrap(),
        ];

        for serialized_frontmatter in test_cases.iter() {
            let content = format!("{}", serialized_frontmatter);

            println!("Content:\n{}\n", &content);

            let (frontmatter, body) = parse_content::<TestFrontMatter>(&content).unwrap();

            assert_eq!(body, "");
            assert_eq!(frontmatter, Some(original_frontmatter.clone()));
        }
    }

    #[test]
    /// An empty input string is valid
    fn empty_input() {
        let content = "";

        let (frontmatter, body) = parse_content::<serde_json::Value>(&content).unwrap();

        assert_eq!(body, "");
        assert_eq!(frontmatter, None);
    }

    #[test]
    /// Malformed frontmatter returns an error
    fn malformed_frontmatter() {
        let test_cases = [
            "{",
            r#"{"test":"trailing",}"#,
            "+++\ntest: value\n  asd\n+++",
            "+++\ntest = \n+++",
            "+++\nthis: is yaml \n+++",
            "---\n][\n---",
            "{{:}}",
        ];

        for original_frontmatter in test_cases.iter() {
            let original_body = "this is a file";
            let content = format!("{}\n{}", original_frontmatter, original_body);

            println!("Content:\n{}\n", &content);

            let ret: Result<(Option<serde_json::Value>, _), _> = parse_content(&content);

            println!("Retuned value: {:#?}", ret);

            assert!(ret.is_err());
        }
    }
}
