
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
    fn object_pattern_should_handle_at_pattern() {
        let matcher : fn(&(u8, u8)) -> Vec<(u8, u8, u8, u8)> = object_pattern!((a @ 1, b @ !); c @ x => { (*a, *b, *x, *c) });
        let output = matcher(&(1, 2));
        assert_eq!( output, [(1, 2, 2, 2)] );
    }

    #[test]
    fn object_pattern_should_handle_empty_tuple() {
        let matcher : fn(Option<()>) -> Vec<char> = object_pattern!(Some(!); () => { 's' });
        let output = matcher(Some(()));
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_last_tuple() {
        let matcher : fn(Option<((u8, u8), u8)>) -> Vec<char> = object_pattern!(Some((!, 0)); (1, 1) => { 's' });
        let output = matcher(Some(((1, 1),0)));
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_internal_tuple() {
        let matcher : fn(Option<(u8, u8)>) -> Vec<char> = object_pattern!(Some((!, 0)); 1 => { 's' });
        let output = matcher(Some((1,0)));
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_tuple() {
        let matcher : fn((u8, u8)) -> Vec<char> = object_pattern!((!, 0); 1 => { 's' });
        let output = matcher((1,0));
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_use_variables() {
        struct Options2(u8, u8);
        let matcher : for<'a> fn(&'a Options2) -> Vec<(&'a u8, &'a u8)> = object_pattern!(Options2(a, !); b => { (a, b) });
        let output = matcher(&Options2(1, 2));
        assert_eq!(output.len(), 1);
        assert_eq!(*output[0].0, 1);
        assert_eq!(*output[0].1, 2);
    }

    #[test]
    fn object_pattern_should_handle_param_cons_with_multiple_nexts() {
        struct Options2(u8, u8);
        let matcher : fn(Options2) -> Vec<char> = object_pattern!(Options2(!, !); 4 => { 's' });
        let output = matcher(Options2(4, 4));
        assert_eq!(output, vec!['s', 's']);
    }

    #[test]
    fn object_pattern_should_handle_param_cons_with_next() {
        struct Options2(u8, u8);
        let matcher : fn(Options2) -> Vec<char> = object_pattern!(Options2(!, 2); 4 => { 's' });
        let output = matcher(Options2(4, 2));
        assert_eq!(output, vec!['s']);
    }

    #[test]
    fn object_pattern_should_handle_nested_param_cons() {
        struct Options3(u8, u8);
        struct Options2(Options3, Options3);
        let matcher : fn(Options2) -> Vec<char> = object_pattern!(Options2(Options3(1, 2), Options3(3, 4)) => { 's' });
        let output = matcher(Options2(Options3(1, 2), Options3(3, 4)));
        assert_eq!(output, vec!['s']);
    }

    #[test]
    fn object_pattern_should_handle_param_cons() {
        struct Options2(u8, u8);
        let matcher : fn(Options2) -> Vec<char> = object_pattern!(Options2(1, 2) => { 's' });
        let output = matcher(Options2(1, 2));
        assert_eq!(output, vec!['s']);
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