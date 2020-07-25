pub fn map<A, B, F>(input: Vec<A>, mut f: F) -> Vec<B>
    where F: FnMut(A) -> B{
    let mut res = Vec::new();

    for v in input {
        res.push(f(v));
    }

    res
}