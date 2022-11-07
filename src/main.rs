
use bending::*;


fn main() {
    //let _matcher : fn(Tree) -> Vec<char> = object_pattern!( Tree::Leaf(_) => { 's' });
        
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    enum Tree {
        Node(Box<Tree>, Box<Tree>),
        Leaf(u8),
    }

    #[derive(Debug)]
    enum Options {
        First,
        Second,
    }

    #[test]
    fn object_pattern_should_handle_namespaceless_cons() {
        use Options::*;
        let matcher : fn(Options) -> Vec<char> = object_pattern!( First => { 's' });
        let output = matcher(First);
        assert_eq!(output, vec!['s']);
    }

    #[test]
    fn object_pattern_should_handle_single_pattern() {
        let matcher : fn(usize) -> Vec<char> = object_pattern!(5 => { 's' });
        let output = matcher(5);
        assert_eq!(output, vec!['s']);
    }

    #[test]
    fn object_pattern_should_next_pattern() {
        let matcher : fn(usize) -> Vec<char> = object_pattern!(!; 5 => { 's' });
        let output = matcher(5);
        assert_eq!(output, vec!['s']);
    }
}