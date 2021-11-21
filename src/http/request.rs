use super::error::ParseError;
use super::method::Method;
use super::Query;
use std::convert::TryFrom;
use std::result::Result;
use std::str;

#[derive(Debug)]
pub struct Request<'buff> {
    path: &'buff str,
    query: Option<Query<'buff>>,
    method: Method,
}

impl<'buff> Request<'buff> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl<'buff> TryFrom<&'buff [u8]> for Request<'buff> {
    type Error = ParseError;

    fn try_from(buf: &'buff [u8]) -> Result<Request<'buff>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query = None;
        if let Some(i) = path.find('?') {
            query = Some(Query::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}
