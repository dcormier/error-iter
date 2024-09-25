use error_iter::ErrorIterator;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("I/O Error")]
    Io(#[from] io::Error),

    #[error("Unknown error")]
    _Unknown,
}

fn main() {
    let error = io::Error::new(io::ErrorKind::Other, "oh no!");

    let errors = error_strings(&error);
    assert_eq!(vec!["oh no!"], errors);
    eprintln!("Errors: {errors:#?}");

    let error = Error::from(error);

    let errors = error_strings(&error);
    assert_eq!(vec!["I/O Error", "oh no!"], errors);
    eprintln!("Errors: {errors:#?}");
}

fn error_strings(err: &dyn std::error::Error) -> Vec<String> {
    ErrorIterator::new(err).map(|err| err.to_string()).collect()
}
