// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // the error here is that vec is an immutable variable and we are trying to mutate it
    // to fix this, we have to make vec mutable
    // while vectors are arrays that are growable (as opposed to normal arrays)
    // -- we still need to explicitly give them a mutable binding to be able to access it
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
