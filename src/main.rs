
use bending::*;

fn main() {
    /*object_pattern!(_);
    object_pattern!(5);
    object_pattern!(5.1);
    object_pattern!("blah");
    object_pattern!("blah";5;_);
    object_pattern!(!; !);*/
    let output : for<'a> fn(&'a usize) -> Vec<usize> = object_pattern!(!; !; 5 => { 0usize });

    let z = output(&5);
        
    println!("Hello, world! {:?}", z);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn object_pattern_should_handle_single_pattern() {
        let matcher : fn(usize) -> Vec<char> = object_pattern!(5 => { 's' });
        let output = matcher(5);
        assert_eq!(output, vec!['s']);
    }
}