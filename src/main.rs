
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
    fn object_pattern_should_handle_namespace_in_module_cons() {
        mod inner {
            #[derive(Debug)]
            pub enum Options2 {
                First,
                Second,
            }
        }
        let matcher : fn(inner::Options2) -> Vec<char> = object_pattern!(inner::Options2::First => { 's' });
        let output = matcher(inner::Options2::First);
        assert_eq!(output, vec!['s']);
    }

    #[test]
    fn object_pattern_should_handle_namespace_cons() {
        let matcher : fn(Options) -> Vec<char> = object_pattern!(Options::First => { 's' });
        let output = matcher(Options::First);
        assert_eq!(output, vec!['s']);
    }

    #[test]
    fn object_pattern_should_handle_namespaceless_cons_after_next() {
        use Options::*;
        let matcher : fn(Options) -> Vec<char> = object_pattern!( !; First => { 's' });
        let output = matcher(First);
        assert_eq!(output, vec!['s']);
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