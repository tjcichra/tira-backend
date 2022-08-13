use crate::controller::{self, TiraErrorResponse};
use std::cmp::Ordering;

pub mod assignments;
pub mod categories;
pub mod comments;
pub mod emails;
pub mod images;
pub mod security;
pub mod sessions;
pub mod tickets;
pub mod users;

pub fn check_only_one_row_changed(rows_changed: usize) -> Result<(), TiraErrorResponse> {
    match rows_changed.cmp(&1) {
        Ordering::Equal => Ok(()),
        Ordering::Less => Err(controller::create_error_response_500(
            "No row was affected".into(),
        )),
        Ordering::Greater => Err(controller::create_error_response_500(
            "More than one row was affected".into(),
        )),
    }
}

pub fn _check_at_least_one_row_changed(rows_changed: usize) -> Result<(), TiraErrorResponse> {
    if let Ordering::Less = rows_changed.cmp(&1) {
        Err(controller::create_error_response_500(
            "No row was affected".into(),
        ))
    } else {
        Ok(())
    }
}
