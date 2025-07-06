fn fill_vec(vec: Vec<i32>) -> Vec<i32> {

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

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        // when passing vec0 to a function as an argument, we are passing ownership of that value
        // -- therefore, the error here is that vec0 is not in the scope of this function anymore
        // by adding .clone(), we're creating a deep copy of vec0 and passing that to the funcion 
        // -- which creates a new vector struct on the stack and a separate copy of the data on the heap
        // so the fill_vec function takes ownership of the clone, while the original vec0 in this function remains unchanged
        
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
