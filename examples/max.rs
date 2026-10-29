use vstd::prelude::*;

verus! {

fn max(x: i64, y: i64) -> (m: i64)
    requires
    ensures
        m>=x,
        m>=y,
        (m==x) || (m==y)
{
    if x>y { x } else { y }
}

fn main() {}
}
