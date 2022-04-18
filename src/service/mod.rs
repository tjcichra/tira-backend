use crate::controller::{TiraMessage, TiraErrorResponse, self};
use std::cmp::Ordering;

pub mod categories;
pub mod images;
pub mod security;
pub mod sessions;
pub mod tickets;
pub mod users;

pub fn check_only_one_row_changed(
    rows_changed: usize,
) -> Result<(), TiraErrorResponse> {
    match rows_changed.cmp(&1) {
        Ordering::Equal => Ok(()),
        Ordering::Less => Err(controller::create_error_response_500("No row was affected".into())),
        Ordering::Greater => Err(controller::create_error_response_500("More than one row was affected".into())),
    }
}

pub fn check_at_least_one_row_changed<E: Into<TiraMessage>>(
    result: Result<usize, E>,
) -> Result<(), TiraMessage> {
    let rows_affected = match result {
        Ok(num) => num,
        Err(error) => return Err(error.into()),
    };

    if rows_affected <= 0 {
        Err(TiraMessage {
            message: "No row was affected".to_string(),
        })
    } else {
        Ok(())
    }
}
