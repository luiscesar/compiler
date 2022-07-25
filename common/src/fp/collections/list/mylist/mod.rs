use std::rc::Rc;


#[derive(Debug,PartialEq)]
pub enum MyList<T> {
    Nil(),
    Cons(Rc<T>,Box<MyList<T>>),

} 

impl <T> MyList<T> {
    pub fn head(list:MyList<T>) -> Option<Rc<T>> {
        match list {
            MyList::Nil() => None,
            MyList::Cons(x, tail) => Some(Rc::clone(&x))
        }
    }

    pub fn tail(list:MyList<T>) -> Option<Box<MyList<T>>> {
        match list {
            MyList::Nil() => None,
            MyList::Cons(x, tail) => Some(tail),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyList;
    use std::rc::Rc;

    #[derive(Debug,PartialEq)]
    struct IpAddrInfo(i32,String);
    #[derive(Debug,PartialEq)]
    struct IpAddr(IpAddrInfo);

    #[test]
    fn test_fp_mylist_head_case1() {
        let mylist:MyList<String> = MyList::Nil();
        let head = MyList::head(mylist);
        assert_eq!(head,None);
    }

    #[test]
    fn test_fp_mylist_head_case2() {
        let mylist:MyList<IpAddr> = MyList::Nil();
        let head = MyList::head(mylist);
        assert_eq!(head,None);
    }

    #[test]
    fn test_fp_mylist_head_case3() {
        let x = String::from("x");
        let rc_x = Rc::new(x);
        let mylist:MyList<String> =  
            MyList::Cons(Rc::clone(&rc_x), Box::new(MyList::Nil()));

        let head = MyList::head(mylist);
        let expected_head = Some(Rc::clone(&rc_x));
        assert_eq!(head,expected_head);
    }

    #[test]
    fn test_fp_mylist_head_case4() {
        let x = String::from("x");
        let y:i32 = 123;
        let ip_addr_info = IpAddrInfo{0:123,1:String::from("x")};
        let ip_addr = IpAddr{0:ip_addr_info};
        let rc_ip_addr = Rc::new(ip_addr);
        let mylist:MyList<IpAddr> =  
            MyList::Cons(Rc::clone(&rc_ip_addr), Box::new(MyList::Nil()));

        let head = MyList::head(mylist);
        let expected_head = Some(Rc::clone(&rc_ip_addr));
        assert_eq!(head,expected_head);
    }

    #[test]
    fn test_fp_mylist_tail_case1() {
        let mylist:MyList<String> = MyList::Nil();
        let tail = MyList::tail(mylist);
        let expected_tail = None;
        assert_eq!(tail,expected_tail);
    }

    #[test]
    fn test_fp_mylist_tail_case2() {
        let mylist:MyList<IpAddr> = MyList::Nil();
        let tail = MyList::tail(mylist);
        let expected_tail = None;
        assert_eq!(tail,expected_tail);
    }

    #[test]
    fn test_fp_mylist_tail_case3() {
        let x = String::from("x");
        let rc_x = Rc::new(x);
        let mylist:MyList<String> =  
            MyList::Cons(Rc::clone(&rc_x), Box::new(MyList::Nil()));

        let tail = MyList::tail(mylist);
        println!("tail {:?}", tail);
    }

    #[test]
    fn test_fp_mylist_tail_case4() {
        let x = String::from("x");
        let rc_x = Rc::new(x);

        let y = String::from("y");
        let rc_y = Rc::new(y);

        let mylist:MyList<String> =  
            MyList::Cons(Rc::clone(&rc_x), Box::new(MyList::Nil()));

        let mylist2:MyList<String> =  
            MyList::Cons(Rc::clone(&rc_y), Box::new(mylist));
        
        let tail = MyList::tail(mylist2);
        println!("tail {:?}", tail);
    }
}

