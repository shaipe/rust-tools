/// 
use std::collections::BTreeMap;


fn main() {
    let data = vec![(1, 2), (2, 3), (1, 1), (2, 4), (3, 5)]; 
    let grouped = vec![(1, vec![2, 1]), (2, vec![3, 4]), (3, vec![5])]; 
    assert_eq!(group_pairs(data).into_iter().collect::<Vec<_>>(), grouped);
}

fn group_pairs<A, B, I>(v: I) -> BTreeMap<A, Vec<B>>
where
    A: Ord,
    I: IntoIterator<Item = (A, B)>,
{
    let mut result = BTreeMap::<A, Vec<B>>::new();
    for (a, b) in v {
        result.entry(a).or_default().push(b);
    }
    result
}
