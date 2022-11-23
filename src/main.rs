
use bending::*;


fn main() {
        
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
    fn object_pattern_should_handle_list_with_variables() {
        let matcher : fn(&[u8]) -> Vec<(u8, u8, u8)> 
            = object_pattern!([a, b, c] => { (*a, *b, *c) });
        let output = matcher(&vec![1, 2, 3]);
        assert_eq!( output, [(1, 2, 3)] );
    }

    #[test]
    fn object_pattern_should_handle_list_with_at_rest() {
        let matcher : fn(&[u8]) -> Vec<char> 
            = object_pattern!([1..=10, 2 | 3, a @ ..] ? { a.len() == 4 } => { 's' });
        let output = matcher(&vec![1, 2, 3, 4, 5, 6]);
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_list_with_rest() {
        let matcher : fn(&[u8]) -> Vec<u8> 
            = object_pattern!([1..=10, 2 | 3, a @ !, ..]; 3  => { *a });
        let output = matcher(&vec![1, 2, 3, 4, 5, 6]);
        assert_eq!( output, [3] );
    }

    #[test]
    fn object_pattern_should_handle_list_with_other_patterns() {
        let matcher : fn(&[u8]) -> Vec<u8> 
            = object_pattern!([1..=10, 2 | 3, a @ !]; 3  => { *a });
        let output = matcher(&vec![1, 2, 3]);
        assert_eq!( output, [3] );
    }

    #[test]
    fn object_pattern_should_handle_list_with_at_pattern() { // Note:  Important because of 'x @ ..'
        let matcher : fn(&[u8]) -> Vec<u8> 
            = object_pattern!([1, 2, a @ _] => { *a });
        let output = matcher(&vec![1, 2, 3]);
        assert_eq!( output, [3] );
    }

    #[test]
    fn object_pattern_should_handle_list() {
        let matcher : fn(&[u8]) -> Vec<char> 
            = object_pattern!([1, 2, 3] => { 's' });
        let output = matcher(&vec![1, 2, 3]);
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_empty_list() {
        let matcher : fn(&[u8]) -> Vec<char> 
            = object_pattern!([] => { 's' });
        let output = matcher(&vec![]);
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_negative_range() {
        let matcher : fn(i8) -> Vec<i8> 
            = object_pattern!(x @ -1..=1 => { x });
        let output = matcher(0);
        assert_eq!( output, [0] );
    }

    #[test]
    fn object_pattern_should_handle_negative_literal() {
        let matcher : fn(i8) -> Vec<char> 
            = object_pattern!(-1 => { 's' });
        let output = matcher(-1);
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_struct_with_rest() {
        struct X {
            y : u8,
            s : u8,
        }
        let matcher : fn(X) -> Vec<char> 
            = object_pattern!(X { y: 0, .. } => { 's' });
        let output = matcher(X { s: 0, y: 0 } );
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_struct_with_only_rest() {
        struct X {
            s : u8 
        }
        let matcher : fn(X) -> Vec<char> 
            = object_pattern!(X { .. } => { 's' });
        let output = matcher(X { s: 0 } );
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_multiple_struct_and_if() {
        struct X {
            s : S
        }
        struct S {
            x : u8,
            y : u8,
            z : u8,
        }
        let matcher : fn(&X) -> Vec<char> 
            = object_pattern!(X { s: w @ ! } { matches!(w, S { .. }) }; S { x : 0, y : 1, z: 2 } => { 's' });
        let output = matcher(&X { s: S { x: 0, y: 1, z: 2 } } );
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_multiple_struct() {
        struct X {
            s : S
        }
        struct S {
            x : u8,
            y : u8,
            z : u8,
        }
        let matcher : fn(X) -> Vec<char> 
            = object_pattern!(X { s: ! }; S { x : 0, y : 1, z: 2 } => { 's' });
        let output = matcher(X { s: S { x: 0, y: 1, z: 2 } } );
        assert_eq!( output, ['s'] );
    }

    #[test]
    fn object_pattern_should_handle_single_struct() {
        struct X {
            s : u8 
        }
        let matcher : fn(X) -> Vec<u8> 
            = object_pattern!(X { s: y } => { y });
        let output = matcher(X { s: 0 } );
        assert_eq!( output, [0] );
    }

    #[test]
    fn object_pattern_should_handle_struct() {
        struct X {
            s : S
        }
        struct S {
            x : u8,
            y : u8,
            z : u8,
        }
        let matcher : fn(X) -> Vec<u8> 
            = object_pattern!(X { s: ! }; S { x, y: 8, .. } => { x });
        let output = matcher(X { s: S { x: 1, y: 8, z: 0 }});
        assert_eq!( output, [1] );
    }

    #[test]
    fn object_pattern_should_handle_or() {
        let matcher : fn((u8, u8)) -> Vec<(u8, u8)> 
            = object_pattern!((a @ 1 | a @ 2, !) { a != 0 }; x | x => { (a, x) });
        let output = matcher((2, 3));
        assert_eq!( output, [(2, 3)] );
    }

    #[test]
    fn object_pattern_should_handle_if() {
        let matcher : fn(u8) -> Vec<(u8, u8)> = object_pattern!(a @ ! {a != 0}; b @ 5..=10 {b != 7} => { (a, b) });
        let output = matcher(6);
        assert_eq!( output, [(6, 6)] );
    }

    #[test]
    fn object_pattern_should_handle_ranges() {
        let matcher : fn((u8, char)) -> Vec<(u8, char)> 
            = object_pattern!((a @ 1..=10, !); b @ 'A'..='H' => { (a, b) });
        let output = matcher((4, 'D'));
        assert_eq!( output, [(4, 'D')] );
    }

    #[test]
    fn object_pattern_should_handle_char_literal() {
        let matcher : fn(char) -> Vec<()> = object_pattern!('a' => { () });
        let output = matcher('a');
        assert_eq!( output, [()] );
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