use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum RequestMethod {
    GET(String),
    HEAD(String),
    POST(String),
    PUT(String),
    DELETE(String),
    CONNECT(String),
    OPTIONS(String),
    TRACE(String),
    PATCH(String),
}

#[derive(Debug, PartialEq)]
struct RequestHeaders(HashMap<String, String>);

impl FromStr for RequestHeaders {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Request {
    method: RequestMethod,
    version: String,
    headers: RequestHeaders,
    body: String,
}

impl FromStr for Request {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split("\n\n");

        let headers = s.next().ok_or("Could not parse header from request")?;

        let mut headers = headers.lines();

        let firstline = headers
            .next()
            .ok_or("Could not parse method, version, or path")?;
        let (method, path_version) = firstline
            .split_once(" ")
            .ok_or("Could not parse request method")?;
        let (path, version) = path_version
            .split_once(" ")
            .ok_or("Could not parse path or http version")?;

        let path = path.to_owned();
        let method = match method {
            "GET" =>     Ok(RequestMethod::GET(path)),
            "HEAD" =>    Ok(RequestMethod::HEAD(path)),
            "POST" =>    Ok(RequestMethod::POST(path)),
            "PUT" =>     Ok(RequestMethod::PUT(path)),
            "DELETE" =>  Ok(RequestMethod::DELETE(path)),
            "CONNECT" => Ok(RequestMethod::CONNECT(path)),
            "OPTIONS" => Ok(RequestMethod::OPTIONS(path)),
            "TRACE" =>   Ok(RequestMethod::TRACE(path)),
            "PATCH" =>   Ok(RequestMethod::PATCH(path)),
            _ => Err("Not a valid http 1.1 request method"),
        }?;
        let version = version.to_owned();

        let headers: Option<HashMap<&str, &str>> = headers.map(|s| s.split_once(": ")).collect();

        let headers = headers.ok_or("Could not parse individual headers from the header")?;
        let headers: HashMap<String, String> = headers
            .iter()
            .map(|(&x, &y)| (x.to_owned(), y.to_owned()))
            .collect();
        let headers = RequestHeaders(headers);
        let body = s.next().ok_or("Could not parse body")?;
        let body = body.to_owned();

        Ok(Self {
            method,
            version,
            headers,
            body,
        })
    }
}

mod test {
    use super::*;

    #[test]
    fn request_fromstr() {
        let request: &'static str = r#"GET /home.html HTTP/1.1
Host: developer.mozilla.org
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:50.0) Gecko/20100101 Firefox/50.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Referer: https://developer.mozilla.org/testpage.html
Connection: keep-alive
Upgrade-Insecure-Requests: 1
If-Modified-Since: Mon, 18 Jul 2016 02:36:04 GMT
If-None-Match: "c561c68d0ba92bbeb8b0fff2a9199f722e3a621a"
Cache-Control: max-age=0

abcd"#;

        let method = RequestMethod::GET("/home.html".to_owned());
        let version = "HTTP/1.1".to_owned();
        let headers: HashMap<String, String> = HashMap::from_iter([
            ("Host", "developer.mozilla.org"),
            ("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:50.0) Gecko/20100101 Firefox/50.0"),
            ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
            ("Accept-Language", "en-US,en;q=0.5"),
            ("Accept-Encoding", "gzip, deflate, br"),
            ("Referer", "https://developer.mozilla.org/testpage.html"),
            ("Connection", "keep-alive"),
            ("Upgrade-Insecure-Requests", "1"),
            ("If-Modified-Since", "Mon, 18 Jul 2016 02:36:04 GMT"),
            ("If-None-Match", "\"c561c68d0ba92bbeb8b0fff2a9199f722e3a621a\""),
            ("Cache-Control", "max-age=0"),
        ].iter().map(|(x,y)| (x.to_string(), y.to_string())));

        let headers = RequestHeaders(headers);
        let body = "abcd".to_owned();

        let expected = Request {
            method,
            version,
            headers,
            body,
        };

        let actual = Request::from_str(request).unwrap();

        assert_eq!(expected, actual)
    }
}
