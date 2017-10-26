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
        vec!["placeholder"]
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

}
