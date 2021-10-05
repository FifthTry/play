pub use crate::*;
pub use chrono::prelude::*;
pub use chrono::{DateTime, Utc};
pub use realm::base::*;
pub use realm::{Or404, Page as RealmPage};
pub fn datetime(d: DateTime<Utc>) -> DateTime<Utc> {
    d
}
