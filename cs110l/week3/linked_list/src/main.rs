use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    let mut list_cloned = list.clone();
    println!("{}", list_cloned);
    list_cloned.push_front(666);
    println!("{}", list);
    println!("{}", list_cloned);

    println!("{}", list == list_cloned);
    list.push_front(666);
    println!("{}", list == list_cloned);

    let mut l1 = LinkedList::<String>::new();
    l1.push_front(String::from("aa"));
    l1.push_front(String::from("bb"));
    l1.push_front(String::from("cc"));
    let mut l2 = l1.clone();
    for i in l1 {
        println!("{}", i);
    }
    for i in &l2 {
        println!("{}", i);
    }
    println!("{}", l2);
    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
