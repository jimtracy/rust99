use std::fmt::Debug;

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

pub fn pack<'a,T: Eq + Debug>(ls: &[&'a T]) -> Vec<Vec<&'a T>> {
    let mut retval = Vec::new();
    if ls.len()>0 {
        let group: Vec<&'a T> = ls
        .iter()
        .filter_map(|&item| {
            if (*item)==*(ls[0]) {
                Some(item)
            } else {
                None
            }
        })
        //.map(|item| *item)
        .collect();
        let newstart = group.len();
        retval.push(group);
        if newstart<ls.len() {
            retval.extend(pack(&ls[newstart..]));
        }
    }
    retval
}

pub fn rle<'a,T: Eq + Debug>(ls: &[&'a T]) -> Vec<(&'a T,usize)> {
    let mut retval = Vec::new();
    if ls.len()>0 {
        let ((fst,co),_) = ls
        .iter()
        .fold(
            ((ls[0],0),false),
            |((elem,cou),sealed), &item| {
            if !sealed && (*item)==*(elem) {
                ((elem,cou+1),false)
            } else {
                ((elem,cou),true)
            }
        });
        retval.push((fst,co));
        if co<ls.len() {
            retval.extend(rle(&ls[co..]));
        }
    }
    retval
}

#[derive(Debug)]
pub enum RLE<'a,T: 'a + Eq> {
    Elem(&'a T),
    Subls(&'a T,usize)
}

impl<'a,T: Eq> PartialEq for RLE<'a, T> {
    fn eq(&self,other: &RLE<'a,T>) -> bool {
        match *self {
            RLE::Elem(thing) => {
                match *other {
                    RLE::Elem(thing2)=>thing==thing2,
                    _ => false
                }
            },
            RLE::Subls(el,num) => {
                match *other {
                    RLE::Subls(el2,num2) => el==el2 && num==num2,
                    _ => false
                }
            }   
        }
    }
}

impl<'a,T: Eq> Eq for RLE<'a, T> {

}

pub fn rle2<'a,T: Eq + Debug>(ls: &[&'a T]) -> Vec<RLE<'a,T>> {
    let first_pass = rle(ls);
    first_pass.iter().map(|&(it,ct)| {
        if ct==1 {
            RLE::Elem(it)
        } else {
            RLE::Subls(it,ct)
        }
    }).collect()
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

#[test]
fn test_pack() {
    let one = &"one";
    let two = &"two";
    let two2 = &"two";
    let three = &"three";

    assert_eq!(vec![vec![one],vec![two,two2],vec![three]],pack(&[one, two, two2, three]));
}

#[test]
fn test_rle() {
    let a = &"a";
    let b = &"b";
    let c = &"c";
    let d = &"d";

    assert_eq!(vec![(a,3),(b,2),(a,4),(c,3),(d,1)],rle(&[a,a,a,b,b,a,a,a,a,c,c,c,d]));
}

#[test]
fn test_rle2() {
    let a = &"a";
    let b = &"b";
    let c = &"c";
    let d = &"d";

    assert_eq!(vec![RLE::Subls(a,3),RLE::Subls(b,2),RLE::Subls(a,4),RLE::Subls(c,3),RLE::Elem(d)],rle2(&[a,a,a,b,b,a,a,a,a,c,c,c,d]));
}


