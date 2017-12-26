mod api;
mod lz4;
mod place;
mod rules;

pub use self::place::{create_place_file, LuaConfigEntry, LuaConfigValue};
pub use self::rules::{VerificationRule, VerificationSet};

// TODO: Cache usernames somehow.

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct RobloxUserID(pub u64);
impl RobloxUserID {
    pub fn for_username(name: &str) -> ::errors::Result<RobloxUserID> {
        api::for_username(name)
    }

    pub fn lookup_username(&self) -> ::errors::Result<String> {
        api::lookup_username(*self)
    }
}