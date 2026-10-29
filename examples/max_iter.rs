use vstd::prelude::*;

verus! {

fn max(values: &[i64]) -> (m: i64)
    requires
        values.len() > 0
    ensures
        forall |k: int| 0<=k && k<values.len() ==> values[k] <= m,
        values@.to_set().contains(m)
{
    let mut max = values[0];
    let mut i = 1;

    while i < values.len()
        invariant
            forall |k: int| 0<=k && k<i ==> values[k] <= max,
            values@.to_set().contains(max),
        decreases
            values.len() - i
    {
        if values[i] > max
        {
            max = values[i]
        }
        i = i + 1;
    }
    max
}

fn main() {}
}
