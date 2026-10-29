use vstd::prelude::*;

verus! {

fn max(values: &[u64]) -> (m: u64)
    requires values.len() > 0,
    ensures
        forall|k: int| 0 <= k < values.len() ==> values[k] <= m,
        exists|k: int| 0 <= k < values.len() && values[k] == m,
{
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

fn main() {}
}
