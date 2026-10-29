use std::collections::HashMap;
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

// A named predicate keeps all frequency expressions identical to the solver.
spec fn matches(value: u64) -> spec_fn(u64) -> bool {
    |x: u64| x == value
}

// Extending a prefix adds exactly the pairs involving its new last element.
proof fn lemma_pairs_push(prefix: Seq<u64>, value: u64)
    ensures
        spec_equal_pairs(prefix.push(value)) == spec_equal_pairs(prefix)
            + prefix.filter(matches(value)).len(),
{
    assert(prefix.push(value).drop_last() =~= prefix);
    assert(prefix.push(value).last() == value);
    assert((|x: u64| x == prefix.push(value).last()) =~= matches(value));
}

// A prefix cannot contain more equal pairs than the whole input.
proof fn lemma_pairs_prefix(s: Seq<u64>, end: int)
    requires 0 <= end <= s.len(),
    ensures spec_equal_pairs(s.take(end)) <= spec_equal_pairs(s),
    decreases s.len() - end,
{
    if end < s.len() {
        lemma_pairs_prefix(s.drop_last(), end);
        assert(s.drop_last().take(end) =~= s.take(end));
    } else {
        assert(s.take(end) =~= s);
    }
}

fn equal_pairs(s: &[u64]) -> (r: u64)
    requires
        spec_equal_pairs(s@) <= u64::MAX
    ensures
        r == spec_equal_pairs(s@)
{
    let mut counts = HashMap::<u64, u64>::new();
    let mut pairs: u64 = 0;
    let mut i: usize = 0;
    while i < s.len()
        invariant
            i <= s.len(),
            spec_equal_pairs(s@) <= u64::MAX,
            pairs == spec_equal_pairs(s@.take(i as int)),
            forall|value: u64| (if counts@.contains_key(value) {
                counts@[value] as nat
            } else { 0 }) == #[trigger] s@.take(i as int).filter(matches(value)).len(),
        decreases s.len() - i,
    {
        let value = s[i];
        let ghost prefix = s@.take(i as int);
        let ghost before = counts@;
        proof {
            assert(s@.take(i + 1) =~= prefix.push(value));
            lemma_pairs_push(prefix, value);
            lemma_pairs_prefix(s@, i + 1);
            prefix.lemma_filter_len(matches(value));
            assert((if before.contains_key(value) { before[value] as nat } else { 0 })
                == prefix.filter(matches(value)).len());
        }
        // One entry lookup, followed by an in-place update (no separate get/insert).
        {
            let count = counts.entry(value).or_insert(0);
            assert(*count == prefix.filter(matches(value)).len());
            pairs = pairs + *count;
            *count = *count + 1;
        }
        proof {
            assert forall|key: u64| (if counts@.contains_key(key) {
                counts@[key] as nat
            } else { 0 }) == #[trigger] s@.take(i + 1).filter(matches(key)).len() by {
                assert((if before.contains_key(key) { before[key] as nat } else { 0 })
                    == prefix.filter(matches(key)).len());
                prefix.lemma_filter_len_push(matches(key), value);
            }
        }
        i = i + 1;
    }
    assert(s@.take(i as int) =~= s@);
    pairs
}

fn main() {}

}
