use std::{time::{Duration, SystemTime}, cmp::Ordering};

use diesel::{result::Error, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

use crate::{
    controller::TiraMessage,
    models::{Login, User},
    TiraDbConn,
};

pub mod categories;
pub mod sessions;
pub mod tickets;
pub mod users;

pub fn check_only_one_row_changed<E: Into<TiraMessage>>(
    result: Result<usize, E>,
) -> Result<(), TiraMessage> {
    let rows_affected = match result {
        Ok(num) => num,
        Err(error) => return Err(error.into()),
    };

    match rows_affected.cmp(&1) {
        Ordering::Equal => Ok(()),
        Ordering::Less => Err(TiraMessage { message: "No row was affected".to_string() }),
        Ordering::Greater => Err(TiraMessage { message: "More than one row was affected".to_string() }),
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
        Err(TiraMessage { message: "No row was affected".to_string() })
    } else {
        Ok(())
    }
}
