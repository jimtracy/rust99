//learnings: recursive data type cannot by-value contain itself.
//has to be a reference. or a box. otherwise the size is not known.
//if you have a recursive type that you plan to be used in a context
//where new memory is allocated (prob nost of the time) it will have to use boxes
//instead of pointers
//unless you're going to give it a new method
//in which case under the hood its using the heap anyway
//you cant send a reference to memory allocated in a context out of that context
#[derive(Debug)]
pub enum Tree<T: Eq> {
    Empty,
    Branch(T, Box<Tree<T>>, Box<Tree<T>>),
}

pub fn check_if_bintree<T: Eq>(tr: Tree<T>) -> bool {
    true
}

pub fn bal_bintree<'a>(node_num: usize) -> Tree<&'a &'static str> {
    use self::Tree::Branch;
    use self::Tree::Empty;

    let x= &"x";

    match node_num {
        0 => Empty,
        //1 => Branch("x", Box::new(Empty), Box::new(Empty)),
        n => {
            let float_n = (n as f64);
            let float_half = (float_n / 2f64);
            let floor_half = float_half.floor();
            if floor_half == float_half {
                Branch(
                    x,
                    Box::new(bal_bintree(floor_half as usize)),
                    Box::new(bal_bintree((floor_half - 1f64) as usize)),
                )
            } else {
                Branch(
                    x,
                    Box::new(bal_bintree(floor_half as usize)),
                    Box::new(bal_bintree(floor_half as usize)),
                )
            }
        }
    }
}

//learnings: ref is used in destructuring when you want to CREATE a reference to something you are matching
//as opposed to matching something that IS a reference
//== acts on references. compares actual values. not pointer math
impl<T: Eq> PartialEq for Tree<T> {
    fn eq(&self,other: &Tree<T>) -> bool {
        match self {
            &Tree::Empty => {
                match other {
                    &Tree::Empty => true,
                    _ => false
                }
            },
            &Tree::Branch(ref elem,ref left,ref right) => {
                match other {
                    &Tree::Branch(ref elem2,ref left2,ref right2) => elem==elem2 && left==left2 && right==right2,
                    _ => false
                }
            }   
        }
    }
}

impl<T: Eq> Eq for Tree<T> {

}

use std::fmt;


impl<T: Eq + fmt::Display> Tree<T> {
    fn fmt_helper(&self, f: &mut fmt::Formatter, lvl: usize) -> fmt::Result {
        let indent = (0..lvl).map(|_| " ").collect::<String>();
        match *self {
            Tree::Empty=> {
                write!(f, "{}{}", indent, "Empty\n")
            },
            Tree::Branch(ref elem,ref lft, ref rt)=> {
                write!(f, "{}{}\n", indent, elem).and_then(|_| {
                    lft.fmt_helper(f, lvl+4)
                }).and_then(|_| {
                    rt.fmt_helper(f, lvl+4)    
                })
            }
        }
    }
}

impl<T: fmt::Display + Eq> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_helper(f,0)
    }
}



#[test]
pub fn test_check_bintree() {
    use self::Tree::Branch;
    use self::Tree::Empty;
    let one = &"one";
    let two = &"two";
    let t = Branch(
        one,
        Box::new(Branch(two, Box::new(Empty), Box::new(Empty))),
        Box::new(Empty)
    );
    assert!(check_if_bintree(t))
}

#[test]
//learnings: Box will prevent branch references from dying
//things on the heap are not destroyed when function completes
//they are destroyed when their parent is destroyed
fn test_bal_bintree() {
    use self::Tree::Branch;
    use self::Tree::Empty;
    let x = &"x";
    let lft = Branch(
            x,
            Box::new(Branch(
                x,
                Box::new(Branch(x, Box::new(Empty), Box::new(Empty))),
                Box::new(Branch(x, Box::new(Empty), Box::new(Empty))),
            )),
            Box::new(Branch(
                x,
                Box::new(Branch(x,Box::new(Empty),Box::new(Empty))),
                Box::new(Empty)
            )),
        );
    let rt = bal_bintree(6);
    println!("left:\n{}\n",lft);
    println!("right:\n{}\n",rt);
    assert_eq!(
        lft,
        rt
    );
}
