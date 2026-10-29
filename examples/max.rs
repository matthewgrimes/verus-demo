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

fn max3(x: i64, y: i64, z: i64) -> (m: i64)
    requires
    ensures
        m>=x,
        m>=y,
        m>=z,
        m==x || m==y || m==z
{
    max(max(x,y),z)
}

fn main() {}
}
