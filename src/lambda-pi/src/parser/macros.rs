macro_rules! match_token {
    ($pattern: pat) => {
        |expr| match expr {
            $pattern => Some(()),
            _ => None,
        }
    };

    ($pattern: pat => $then: expr) => {
        |expr| match expr {
            $pattern => Some($then),
            _ => None,
        }
    };
}

macro_rules! consume {
    ($self: expr, $pattern: pat) => { $self.consume(match_token!($pattern)) };
    ($self: expr, $pattern: pat  => $then: expr) => { $self.consume(match_token!($pattern => $then)) }
}

pub(crate) use consume;
pub(crate) use match_token;
