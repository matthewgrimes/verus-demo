use vstd::prelude::*;

verus! {

pub open spec fn spec_equal_pairs(s: Seq<u64>) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else {
        let prefix = s.drop_last();

        spec_equal_pairs(prefix)
            + prefix.filter(|x: u64| x == s.last()).len()
    }
}

fn equal_pairs(s: &[u64]) -> (r: u64)
    requires
        spec_equal_pairs(s@) <= u64::MAX
    ensures
        r == spec_equal_pairs(s@)
{
    0
}

fn main() {}

}
