use std::vec::Vec;


#[derive(Debug,PartialEq)]
pub struct MyStructList<T> {
    values:Vec<T>,
}

impl<T> MyStructList<T> {

    pub fn nil() -> MyStructList<T> {
        let vals:Vec<T> = Vec::with_capacity(10_000_000);
        MyStructList{values:vals}
    }

    pub fn cons(h:T,mut t:MyStructList<T>) -> MyStructList<T> {
        t.values.push(h);
        MyStructList { values: t.values }
    }

    pub fn is_empty(list:MyStructList<T>) -> bool {
        list.values.is_empty()
    }

    pub fn head<'a>(list:&'a MyStructList<T>) -> Option<&'a T> {
        if list.values.is_empty() {
            None
        } else {
            Some(&(list.values[list.values.len()-1]))
        }
    }

    pub fn tail(mut list:MyStructList<T>) -> MyStructList<T> {
        if list.values.len() <= 1 {
            MyStructList::nil()
        } else {
            list.values.pop();
            MyStructList{values:list.values}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyStructList;
    use std::vec::Vec;

    #[derive(Debug,PartialEq)]
    struct IpAddrInfo(i32,String);

    #[derive(Debug,PartialEq)]
    struct IpAddr(IpAddrInfo);

    #[test]
    fn test_fp_mystructlist_case1() {
        let mylist:MyStructList<String> = MyStructList { values: Vec::new() };
        let mylist2:MyStructList<String> = MyStructList { values: Vec::new() };
        assert_eq!(mylist,mylist2);
    }

    #[test]
    fn test_fp_mystructlist_case2() {
        let mylist:MyStructList<String> = 
            MyStructList { values: Vec::from([String::from("Hello")])};
        let mylist2:MyStructList<String> = 
            MyStructList { values: Vec::from([String::from("Hello")])};        
        assert_eq!(mylist,mylist2);
    }

    #[test]
    fn test_fp_mystructlist_case3() {
        let mylist:MyStructList<IpAddr> = MyStructList { values: Vec::new() };
        let mylist2:MyStructList<IpAddr> = MyStructList { values: Vec::new() };
        assert_eq!(mylist,mylist2);
    }

    #[test]
    fn test_fp_mystructlist_case4() {
        let ip_addr_info1 = 
            IpAddrInfo{0:1,1:String::from("Hello")};
        let ip_addr1 = IpAddr{0:ip_addr_info1};
        
        let ip_addr_info2 = 
            IpAddrInfo{0:1,1:String::from("Hello")};
        let ip_addr2 = IpAddr{0:ip_addr_info2};

        let mylist:MyStructList<IpAddr> = MyStructList { values: Vec::from([ip_addr1])};
        let mylist2:MyStructList<IpAddr> = MyStructList { values: Vec::from([ip_addr2])};
        assert_eq!(mylist,mylist2);
    }
}