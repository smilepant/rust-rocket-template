use std::collections::HashMap;

use std::sync::Mutex;

use crate::models::user::User;

pub type UserMap = Mutex<HashMap<u64, User>>;