macro_rules! filter {
    ($e: expr) => {
        if !$e {
            return None;
        }
    };
}

pub(crate) use filter;
