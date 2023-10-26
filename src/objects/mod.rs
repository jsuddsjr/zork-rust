enum Flags {
    READBIT = 1 << 0,
    TAKEBIT = 1 << 1,
    DOORBIT = 1 << 2,
    NDESCBIT = 1 << 3,
    LIGHTBIT = 1 << 4,
    ACTORBIT = 1 << 5,
    ONBIT = 1 << 6,
    OPENBIT = 1 << 7,
    LOCKBIT = 1 << 8,
    TOUCHBIT = 1 << 9,
    RSEENBIT = 1 << 10,
    RLIGHTBIT = 1 << 11,
    RLANDBIT = 1 << 12,
    RHOUSEBIT = 1 << 13,
    RENDGAME = 1 << 14,
    INVISIBLE = 1 << 15,
    CONTBIT = 1 << 16,
    TRYTAKEBIT = 1 << 17,
}

struct Object {
    desc: String,
    loc: String,
    flags: u64,
}

impl Object {
    fn new(desc: &'static str, loc: &'static str, flags: u64) -> Object {
        Object {
            desc: desc.to_string(),
            loc: loc.to_string(),
            flags,
        };
    }

    fn set_loc(&mut self, loc: String) {
        self.loc = loc;
    }

    fn set_flag(&mut self, flag: u64) {
        self.flags &= flag;
    }

    fn clear_flag(&mut self, flag: u64) {
        self.flags &= !flag;
    }

    fn has_flag(&self, flag: u64) -> bool {
        self.flags & flag == flag
    }
}

