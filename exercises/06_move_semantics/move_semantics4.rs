fn main() {
    // You can optionally experiment here.
 let mut x = Vec::new();
        let y = &mut x;
        println!("y: {:?}", y);
        y.push(42);
        println!("y: {:?}", y);
        let z = &mut x;
        //y.push(42);
        println!("z: {:?}", z);
        z.push(13);
        println!("z: {:?}",z );
        assert_eq!(x, [42, 13]);

}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        println!("y: {:p}", y);
        y.push(42);
        println!("y: {:p}", y);
        let z = &mut x;
        //y.push(42);
        println!("z: {:p}", z);
        z.push(13);
        println!("z: {:p}",z );
        assert_eq!(x, [42, 13]);
    }
}
