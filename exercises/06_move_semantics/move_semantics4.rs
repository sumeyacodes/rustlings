fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        // the issue here is that y and z were trying to borrow from x at the same time
        // the error is that we can't have multiple mutable references to the same value at the same time
        // so to fix this, we initialise z after y finishes borrowing from x

        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);

        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
