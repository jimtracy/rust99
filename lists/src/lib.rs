pub mod lists {

    pub fn my_last(ls: Vec<&str>) -> &str {
        ls[ls.len() - 1]
    }

    pub fn my_but_last(ls: Vec<&str>) -> &str {
        ls[ls.len() - 2]
    }

    pub fn element_at(ls: Vec<&str>, ind: usize) -> &str {
        ls[ind]
    }

    pub fn num_elements(ls: Vec<&str>) -> usize {
        ls.len()
    }

    pub fn reverse_list(ls: Vec<&str>) -> Vec<&str> {
        let mut newls = Vec::new();
        let leng = ls.len();
        for i in 0..leng {
            newls.push(ls[(leng - 1) - i])
        }
        newls
    }

    pub fn is_palindrome(ls: Vec<&str>) -> bool {
        ls == reverse_list(ls.to_vec())
    }


}



#[cfg(test)]
mod tests {
    use lists;

    #[test]
    fn test_my_last() {
        assert_eq!(lists::my_last(vec!["hi", "there", "foo"]), "foo");
    }

    #[test]
    fn test_but_last() {
        assert_eq!(lists::my_but_last(vec!["hi", "there", "foo"]), "there");
    }

    #[test]
    fn test_element_at() {
        assert_eq!(lists::element_at(vec!["hi", "there", "foo"], 1), "there");
    }

    #[test]
    fn test_num_elements() {
        assert_eq!(lists::num_elements(vec!["hi", "there", "foo"]), 3);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            lists::reverse_list(vec!["one", "two", "three"]),
            vec!["three", "two", "one"]
        )
    }

    #[test]
    fn test_is_palindrome() {
        assert!(lists::is_palindrome(vec!["one", "two", "two", "one"]));
    }

}
