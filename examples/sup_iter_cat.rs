use vstd::prelude::*;

verus! {

spec fn is_upper_bound(m: u64, values: &[u64]) -> bool {
        forall|k: int| 0 <= k < values.len() ==> values[k] <= m
}

fn sup(values: &[u64]) -> (m: u64)
    ensures
        is_upper_bound(m, values),
        forall|z: u64| is_upper_bound(z, values) ==> m <= z,
{
    if values.len() == 0
    {
        return u64::MIN;
    }
    let mut best = values[0];
    let mut i: usize = 1;
    while i < values.len()
        invariant
            1 <= i <= values.len(),
            forall|k: int| 0 <= k < i ==> values[k] <= best,
            exists|k: int| 0 <= k < i && values[k] == best,
        decreases values.len() - i,
    {
        if values[i] > best {
            best = values[i];
        }
        i = i + 1;
    }
    best
}

// In the finite, totally ordered set u64, a nonempty list attains its
// least upper bound. This uses only the abstract contract, not the loop.
proof fn sup_in_set_if_nonempty(values: &[u64], m: u64)
    requires
        values.len() > 0,
        is_upper_bound(m, values),
        forall|z: u64| is_upper_bound(z, values) ==> m <= z,
    ensures
        exists|k: int| 0 <= k < values.len() && values[k] == m,
{
    if !(exists|k: int| 0 <= k < values.len() && values[k] == m) {
        // by assumption the first value is <= m
        assert(values[0] <= m);
        // if m == 0 then values[0] == m == 0 and we can take k = 0
        assert (m > 0) by {
            if m == 0 {
                assert(values[0] == 0);
                assert(values[0] == m);
                assert(false);
            }
        };

        // this means m - 1 is also an upper bound
        let predecessor = (m - 1) as u64;
        assert forall|k: int| 0 <= k < values.len()
            implies values[k] <= predecessor by {
            assert(values[k] <= m);
            assert(values[k] != m);
        }
        assert(is_upper_bound(predecessor, values));
        // but m was the LEAST upper bound
        assert(m <= predecessor);
        assert(false);
    }
}

fn main() {
}
}
