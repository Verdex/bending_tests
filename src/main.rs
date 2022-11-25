


fn main() {
        
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use bending::*;
    use denest::*;

    #[derive(Debug, PartialEq)]
    enum Tree {
        Node(Box<Tree>, Box<Tree>),
        Leaf(u8),
    }

    impl<'a> Linearizable<'a> for Tree {
        fn l_next(&'a self) -> Vec<&'a Self> {
            match self {
                Tree::Node(a, b) => vec![a, b],
                Tree::Leaf(_) => vec![],
            }
        }
    }    


    #[derive(Debug)]
    enum Options {
        First,
    }

    #[test]
    fn object_pattern_should_be_able_to_double_match_in_lax() {
        let t = Tree::Node(
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(4)),
                Box::new(Tree::Leaf(5)),
            )),
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(5)),
                Box::new(Tree::Leaf(6)),
            )),
        );

        fn is_leaf_of_5<'a>(input : &'a Tree) -> Vec<&'a Tree> {
            let f : fn(&'a Tree) -> Vec<&'a Tree>
                = object_pattern!(res @ Tree::Leaf(x) ? { *x == 5 } => { res });
            f(input)
        }  

        let node_with_even_leaf_with_sibling_of_5 : for<'a> fn(&'a Tree) -> Vec<&'a Tree>
            = object_pattern!(
                Tree::Node(a @ !, b @ !) 
                    & { 
                        let a_5 = is_leaf_of_5(a); 
                        let b_5 = is_leaf_of_5(b);
                    }; 
                res @ Tree::Leaf(x) ? { *x % 2 == 0 && (a_5.len() != 0 || b_5.len() != 0) }
                => { res });

        let output = t.to_lax().flat_map(|tlet| node_with_even_leaf_with_sibling_of_5(&tlet)).collect::<Vec<&Tree>>();
        assert_eq!( output, [&Tree::Leaf(6), &Tree::Leaf(4)] );

    }

    #[test]
    fn object_pattern_should_handle_result_lifetimes() {
        let t = Tree::Node(
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(2)),
                Box::new(Tree::Leaf(3)),
            )),
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(4)),
                Box::new(Tree::Leaf(6)),
            )),
        );

        let matcher : for <'a> fn(&'a Tree) -> Vec<&'a Tree> 
            = object_pattern!(Tree::Node(!, !); Tree::Node(!, !); res @ Tree::Leaf(x) ? { x % 2 == 0 } => { res });

        let output = matcher(&t);
        assert_eq!( output, [&Tree::Leaf(6), &Tree::Leaf(4), &Tree::Leaf(2)] );
    }

    #[test]
    fn object_pattern_should_handle_nested_nexts() {
        let t = Tree::Node(
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(2)),
                Box::new(Tree::Leaf(3)),
            )),
            Box::new(Tree::Node(
                Box::new(Tree::Leaf(4)),
                Box::new(Tree::Leaf(6)),
            )),
        );

        let matcher : fn(&Tree) -> Vec<u8> 
            = object_pattern!(Tree::Node(!, !); Tree::Node(!, !); Tree::Leaf(x) ? { x % 2 == 0 } => { *x });

        let output = matcher(&t);
        assert_eq!( output, [6, 4, 2] );
    }

    #[test]
    fn object_pattern_should_handle_if_and_execute() {
        let matcher : fn(u8) -> Vec<u8>
            = object_pattern!( w @ ! ? { *w % 2 == 0 } & { let x = 1; }; w2 @ 8 ? { *w2 == 8 } & { let y = 2; } => { x + y });
        let output = matcher(8);
        assert_eq!( output, [3] );
    }

    #[test]
    fn object_pattern_should_handle_execute() {
        let matcher : fn(u8) -> Vec<u8>
            = object_pattern!(! & { let x = 1; }; 8 & { let y = 2; } => { x + y });
        let output = matcher(8);
        assert_eq!( output, [3] );
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
            = object_pattern!(x @ -1..=1 => { *x });
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
            s : u8 
        }
        let matcher : fn(X) -> Vec<char> 
            = object_pattern!(X { .. } => { 's' });
        let output = matcher(X { s: 0 } );
        assert_eq!( output, ['s'] );
    }

    #[test] 
    fn object_pattern_should_handle_if_after_variable() {
        let matcher : fn(u8) -> Vec<u8>
            = object_pattern!(a ? { *a == 1 } => { *a });
        let output = matcher(1);
        assert_eq!( output, [1] );
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
            = object_pattern!(X { s: w @ ! } ? { matches!(w, S { .. }) }; S { x : 0, y : 1, z: 2 } => { 's' });
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
            = object_pattern!(X { s: y } => { *y });
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
            #[allow(dead_code)]
            z : u8,
        }
        let matcher : fn(X) -> Vec<u8> 
            = object_pattern!(X { s: ! }; S { x, y: 8, .. } => { *x });
        let output = matcher(X { s: S { x: 1, y: 8, z: 0 }});
        assert_eq!( output, [1] );
    }

    #[test]
    fn object_pattern_should_handle_or() {
        let matcher : fn((u8, u8)) -> Vec<(u8, u8)> 
            = object_pattern!((a @ 1 | a @ 2, !) ? { *a != 0 }; x | x => { (*a, *x) });
        let output = matcher((2, 3));
        assert_eq!( output, [(2, 3)] );
    }

    #[test]
    fn object_pattern_should_handle_if() {
        let matcher : fn(u8) -> Vec<(u8, u8)> = object_pattern!(a @ ! ? {*a != 0}; b @ 5..=10 ? {*b != 7} => { (*a, *b) });
        let output = matcher(6);
        assert_eq!( output, [(6, 6)] );
    }

    #[test]
    fn object_pattern_should_handle_ranges() {
        let matcher : fn((u8, char)) -> Vec<(u8, char)> 
            = object_pattern!((a @ 1..=10, !); b @ 'A'..='H' => { (*a, *b) });
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