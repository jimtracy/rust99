pub fn my_last<'a,T>(ls: &[&'a T]) -> &'a T {
    ls[ls.len() - 1]
}

pub fn my_but_last<'a,T>(ls: &[&'a T]) -> Vec<&'a T> {
    vec![ls[ls.len()-2],ls[ls.len()-1]]
}

pub fn element_at<'a,T>(ls: &[&'a T], ind: usize) -> &'a T {
    ls[ind]
}

pub fn num_elements<'a,T>(ls: &[&'a T]) -> usize {
    ls.len()
}


//learnings: generic types are assumed to be sized
pub fn reverse_list<'a,T>(ls: &[&'a T]) -> Vec<&'a T> {
    let mut newls = Vec::new();
    let leng = ls.len();
    for i in 0..leng {
        newls.push(ls[(leng - 1) - i])
    }
    newls
}



pub fn is_palindrome<T: Eq>(ls: &[&T]) -> bool {
    let mut retval = true;
    let leng = ls.len();
    for i in 0..leng {
        if ls[(leng - 1) - i]!=ls[i] {
            retval = false;
        }
    }
    retval
}

pub enum Node<'a, T: 'a> {
    One(&'a T),
    Many(&'a [Node<'a, T>]),
}


// learnings: you cant pass out a slice if the slice is made from a structure allocated inside function
pub fn flatten<'a, T>(ls_in: &'a Node<'a, T>) -> Vec<&T> {
    let mut newvec: Vec<&T> = Vec::new();
    match ls_in {
        &Node::One(ref elem) => {
            newvec.push(elem);
        }
        &Node::Many(subls) => {
            //subls.iter().map(|x: &Node<'a, T>| flatten(x)).collect::<Vec<&T>>();

            for el in subls {
                let vec2 = flatten(&el);
                for el2 in vec2 {
                    newvec.push(el2);
                }
            }
        }
    }
    newvec
}

pub fn compress<'a,T: Eq>(ls: &[&'a T]) -> Vec<&'a T> {
    let mut retval = Vec::new();
    if ls.len()>0 {
        retval.push(ls[0]);
    }
    if ls.len()>1 {
        for i in 1..ls.len() {
            if ls[i]!=ls[i-1] {
                retval.push(ls[i]);
            }
        }
    }
    retval
}


#[test]
fn test_my_last() {
    let last = &"foo";
    assert_eq!(my_last(&[&"hi", &"there", last]), last);
}

#[test]
fn test_but_last() {
    let foo = &"foo";
    let there = &"there";
    assert_eq!(my_but_last(&[&"hi", there, foo]), [there,foo]);
}

#[test]
fn test_element_at() {
    let there = &"there";
    assert_eq!(element_at(&[&"hi", there, &"foo"], 1), there);
}

#[test]
fn test_num_elements() {
    assert_eq!(num_elements(&[&"hi", &"there", &"foo"]), 3);
}

//learnings: refs to static ony live till the end of the line
#[test]
fn test_reverse() {
    let one = &"one";
    let two = &"two";
    let three = &"three";
    let list_to_reverse = [one,two,three]; 
    let try_reverse = reverse_list(&list_to_reverse);
    let expected_reverse = [three, two, one]; 
    for i in 0..try_reverse.len() {
        assert_eq!(*try_reverse[i],*expected_reverse[i]);
    }
}

#[test]
fn test_is_palindrome() {
    let one = &"one";
    let two = &"two";
    assert!(is_palindrome(&[one, two, two, one]));
}

#[test]
fn test_flatten() {
    let one = &"one";
    let two = &"two";
    let three = &"three";
    assert_eq!(
        flatten(&Node::Many(&vec![
            Node::Many(&[
                Node::One(one),
                Node::One(two),
            ]),
            Node::One(three),
        ])),
        [one, two, three]
    )
}

#[test]
fn test_compress() {
    let one = &"one";
    let two = &"two";
    assert_eq!(vec![one,two,one],compress(&[one, two, two, one]));
}