use crate::common::*;

use core::result;

pub type Result<T, E = Error> = result::Result<T, E>;
