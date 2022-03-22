#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    pub fn list_contains<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {

        let len = second_list.len();

        len == 0 || first_list.windows(len)
        .any(|sublist| sublist == second_list)
       

}                                                                                                                        
    if first_list.len() == second_list.len() && first_list == second_list {
        Comparison::Equal

    } else if first_list.len() != second_list.len() && list_contains(second_list, first_list) {
        Comparison::Sublist

    } else if list_contains(first_list, second_list){
        Comparison::Superlist

    } else {
        Comparison::Unequal
    }
}
