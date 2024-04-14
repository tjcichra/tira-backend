use anyhow::{anyhow, Result};
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

pub fn check_only_one_row_changed(rows_changed: u64) -> Result<()> {
    match rows_changed.cmp(&1) {
        Ordering::Equal => Ok(()),
        Ordering::Less => Err(anyhow!("No rows affected")),
        Ordering::Greater => Err(anyhow!("More than one row affected")),
    }
}

pub fn _check_at_least_one_row_changed(rows_changed: usize) -> Result<()> {
    if let Ordering::Less = rows_changed.cmp(&1) {
        Err(anyhow!("No rows affected"))
    } else {
        Ok(())
    }
}
