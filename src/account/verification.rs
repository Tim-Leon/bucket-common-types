use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    /// NOTE* can not just cast verification between u32 and i32 because of bit flip
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Verification : i16 {
        const UNVERIFIED = 0b0000_0000_0000_0000;
        const EMAIL = 0b0000_0000_0000_0001;
        const PHONE = 0b0000_0000_0000_0010;
        const TOTP = 0b0000_0000_0000_0100;
    }
}