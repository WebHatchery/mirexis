//! Storefront-demo boundaries shared by game flow and presentation.

pub const MISSION_LIMIT: u32 = 3;

pub const fn is_demo_build() -> bool {
    cfg!(feature = "demo")
}

pub const fn campaign_is_complete(operations_completed: u32) -> bool {
    is_demo_build() && operations_completed >= MISSION_LIMIT
}

#[cfg(test)]
mod tests;
