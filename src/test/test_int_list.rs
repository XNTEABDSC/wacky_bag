use crate::{collections::int_list::IntList, structures::idtrait::index_collection_parial::IndexCollectionPartial};



#[test]
fn it_works() {
    let mut  list=IntList::<String>::new();
    list.add(12, String::from("val1"));
    list.add(6, String::from("val2"));
    list.add(8, String::from("dwawd"));
    println!( "{0}", list[8].as_ref().unwrap() );
    println!( "{0}", list[12].as_ref().unwrap() );
    list.remove(8);
    println!( "{0}", match &list[8] {
        None=>"none",
        Some(s)=>&s
    } );
    println!( "{0}", list[12].as_ref().unwrap() );
}
