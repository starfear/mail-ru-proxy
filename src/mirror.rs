pub enum Mirror {
    Hk,
    Pm,
    Re,
    Tf,
    Wf,
    Yt,
    TirechSo,
}

impl Mirror {
    pub fn unwrap(self) -> &'static str {
        use Mirror::*;

        match self {
            Hk => "2ch.hk",
            Pm => "2ch.pm",
            Re => "2ch.re",
            Tf => "2ch.tf",
            Wf => "2ch.wf",
            Yt => "2ch.Yt",
            TirechSo => "2-ch.so",
        }
    }
}
