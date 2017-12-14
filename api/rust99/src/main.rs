
mod lists;
mod mariatest;
mod bintrees;
use bintrees::Tree::Branch;
use bintrees::Tree::Empty;


#[macro_use]
extern crate mysql;

fn main() {
    //let t = Branch("one", &Branch("two", &Empty, &Empty), &Empty);
    println!("{:?}", bintrees::bal_bintree(0));
    println!("{:?}", bintrees::bal_bintree(1));
    println!("{:?}", bintrees::bal_bintree(2));
    println!("{:?}", bintrees::bal_bintree(3));
    println!("{:?}", bintrees::bal_bintree(4));
    println!("{:?}", bintrees::bal_bintree(5));
    println!("{:?}", bintrees::bal_bintree(6));
}
