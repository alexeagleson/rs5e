use std::fmt::Display;
use std::sync::atomic::{AtomicU64, Ordering};

static INCREMENTAL_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct Id {
    id: u64,
    version: u64,
}

impl Id {
    #[must_use]
    pub const fn from_u64(id: u64) -> Self {
        Self { id, version: 0 }
    }

    pub fn new_incremental() -> Self {
        Self {
            id: INCREMENTAL_ID.fetch_add(1, Ordering::Relaxed),
            version: 0,
        }
    }

    pub fn increment_version(&mut self) {
        self.version += 1;
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.id, self.version)
    }
}
