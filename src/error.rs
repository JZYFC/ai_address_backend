use std::fmt::{Debug, Display, Formatter};
use anyhow::anyhow;

pub struct MyError {
    err: anyhow::Error,
}

impl Debug for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.err, f)
    }
}

impl actix_web::error::ResponseError for MyError {

}

impl From<anyhow::Error> for MyError {
    fn from(err: anyhow::Error) -> Self {
        MyError { err }
    }
}

impl From<actix_form_data::Error> for MyError {
    fn from(err: actix_form_data::Error) -> Self {
        Self {
            err: anyhow!(err)
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        Self {
            err: anyhow!(err)
        }
    }
}