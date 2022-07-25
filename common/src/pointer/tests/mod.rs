use std::{borrow::BorrowMut, rc::Rc, cell::RefCell};

use super::Pointer;

#[derive(Debug,PartialEq)]
struct T1 {
    field1:i32,
    field2:String,
}
impl T1 {
    pub fn field1(&self) -> i32 {self.field1}
    pub fn field2(&self) -> &String {&self.field2}
    pub fn set_field1(&mut self,field1:i32) {
        self.field1 = field1;
    }
    pub fn set_field2(&mut self,field2:String) {self.field2=field2}
}

#[derive(Debug,PartialEq)]
struct T2 {
    id:i32,
    data:T1,
} 

impl T2 {
    pub fn data(&self) -> &T1 {
        &self.data
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn set_id(self:&mut Self, id:i32) {
        self.id = id;
    }
    pub fn set_data(&mut self, data:T1) {
        self.data = data;
    }
}

#[test]
fn test_pointer_new_case1() {
    let mut t1 = T1{field1:1,field2:"suioda".to_string()};
    let mut t2 = T2{id:1,data:t1};
    let mut p = Pointer::new_pointer(t2);
    //(*p).set_id(3);
    let p3 = Pointer::clone(&p);
    let data = (*p).data();
    let data1 = (*p3).data();
    assert_eq!(*data,*data1);


    let mut t1 = T1{field1:1,field2:"suioda".to_string()};
    let mut t2 = T2{id:1,data:t1};
    let mut p2 = Pointer::new_box_pointer(t2);
    (*p2).set_id(3);
    assert_eq!(p2.id(),3);

    let mut t1 = T1{field1:1,field2:"alfa".to_string()};
    let mut t2 = T2{id:1,data:t1};
    let mut t11 = T1{field1:1,field2:"beta".to_string()};
    let mut t12 = T1{field1:1,field2:"beta".to_string()};
    let mut t13 = T1{field1:1,field2:"beta".to_string()};
    let mut t14 = T1{field1:1,field2:"beta".to_string()};
    let mut t15 = T1{field1:1,field2:"beta".to_string()};

    let mut p6 = Pointer::new_mut_pointer(t2);
    assert_eq!((*p6).borrow().id(),1);
    (*p6).borrow_mut().set_data(t11);
    (*p6).borrow_mut().set_id(4);

    assert_eq!((*p6).borrow().id(),4);

    let mut p7 = Pointer::clone(&p6);
    assert_eq!((*p7).borrow().id(),4);
    assert_eq!((*p6).borrow().id(),4);
    
    (*p6).borrow_mut().set_data(t12);
    assert_eq!(*((*p6).borrow().data()),t13);

    (*p7).borrow_mut().set_data(t14);
    assert_eq!(*((*p6).borrow().data()),t15);
}