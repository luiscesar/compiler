#[derive(Debug, PartialEq)]
pub struct MyStructTree<T> {
    root: T,
    children: Vec<MyStructTree<T>>,
}

impl<T: PartialEq> MyStructTree<T> {
    pub fn root(&self) -> Option<&T> {
        Some(&self.root)
    }

    pub fn set_root(&mut self, new_root: T) {
        self.root = new_root;
    }

    pub fn children(&self) -> &Vec<MyStructTree<T>> {
        &(self.children)
    }

    pub fn number_of_children(&self) -> usize {
        self.children.len()
    }

    pub fn add_child(&mut self, child: MyStructTree<T>) {
        self.children.push(child);
    }

    pub fn add(&mut self, child_root: T) {
        let child = MyStructTree {
            root: child_root,
            children: Vec::new(),
        };
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: &MyStructTree<T>) {
        let index = self.children.iter().position(|x| *x == *child);
        match index {
            Some(i) => {
                self.children.remove(i);
            }
            None => {}
        }
    }

    pub fn remove(&mut self, child_root: &T) {
        let index = self.children.iter().position(|x| match x.root() {
            Some(x) => (*x == *child_root),
            None => false,
        });
        match index {
            Some(i) => {
                self.children.remove(i);
            }
            None => {}
        }
    }

    pub fn size(&self) -> usize {
        1 + self
            .children
            .iter()
            .fold(0, |acc, child| acc + child.size())
    }
}

#[cfg(test)]
mod tests {

    use super::MyStructTree;

    #[derive(Debug, PartialEq)]
    struct IpAddrInfo {
        pub ip: i32,
        pub network: String,
    }

    #[derive(Debug, PartialEq)]
    enum IpAddr {
        V4(IpAddrInfo),
        V6(IpAddrInfo),
    }

    fn get_root_case1() -> IpAddr {
        let ip_address_info = IpAddrInfo {
            ip: 1,
            network: String::from("1.1.1.1"),
        };
        let root = IpAddr::V4(ip_address_info);
        root
    }

    fn get_root_case2() -> IpAddr {
        let ip_address_info = IpAddrInfo {
            ip: 1,
            network: String::from("1.1.1.1"),
        };
        let root = IpAddr::V6(ip_address_info);
        root
    }

    #[test]
    fn test_mystructtree_case1() {
        let mytree: MyStructTree<String> = MyStructTree {
            root: String::from("root"),
            children: Vec::new(),
        };
        let mytree2: MyStructTree<String> = MyStructTree {
            root: String::from("root"),
            children: Vec::new(),
        };
        assert_eq!(mytree, mytree2);
    }

    #[test]
    fn test_mystructtree_case2() {
        let root1 = get_root_case1();
        let mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        let root2 = get_root_case1();
        let mytree2 = MyStructTree {
            root: root2,
            children: Vec::new(),
        };

        assert_eq!(mytree, mytree2);
    }

    #[test]
    fn test_mystructtree_root_case1() {
        let root1 = get_root_case1();
        let mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        let root2 = get_root_case1();
        assert_eq!(mytree.root(), Some(&root2));
    }

    #[test]
    fn test_mystructtree_set_root_case1() {
        let root1 = get_root_case1();
        let mut mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        let root2 = get_root_case2();
        mytree.set_root(root2);

        let root2 = get_root_case2();
        assert_eq!(mytree.root(), Some(&root2));
    }

    #[test]
    fn test_mystructtree_children_case1() {
        let root1 = get_root_case1();
        let mut mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        let expected_children: Vec<MyStructTree<IpAddr>> = Vec::new();
        assert_eq!(mytree.children(), &expected_children);
    }

    #[test]
    fn test_mystructtree_children_case2() {
        let root1 = get_root_case1();
        let mut mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };
        let children = Vec::from([mytree]);

        let root2 = get_root_case2();
        let mut mytree2 = MyStructTree {
            root: root2,
            children: children,
        };

        let root1 = get_root_case1();
        let mut mytree3 = MyStructTree {
            root: root1,
            children: Vec::new(),
        };
        let expected_children = Vec::from([mytree3]);
        assert_eq!(mytree2.children(), &expected_children);
    }

    #[test]
    fn test_mystructtree_number_of_children_case1() {
        let root1 = get_root_case1();
        let mut mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        assert_eq!(mytree.number_of_children(), 0);
    }

    #[test]
    fn test_mystructtree_number_of_children_case2() {
        let root1 = get_root_case1();
        let mut mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };
        let children = Vec::from([mytree]);

        let root2 = get_root_case2();
        let mut mytree2 = MyStructTree {
            root: root2,
            children: children,
        };

        assert_eq!(mytree2.number_of_children(), 1);
    }

    #[test]
    fn test_mystructtree_add_child_case1() {
        let root1 = get_root_case1();
        let mut mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        let root2 = get_root_case2();
        let mut mytree2 = MyStructTree {
            root: root2,
            children: Vec::new(),
        };

        mytree2.add_child(mytree);

        let root1 = get_root_case1();
        let mut mytree3 = MyStructTree {
            root: root1,
            children: Vec::new(),
        };
        let expected_children = Vec::from([mytree3]);
        assert_eq!(mytree2.children(), &expected_children);
    }

    #[test]
    fn test_mystructtree_add_case1() {
        let root1 = get_root_case1();

        let root2 = get_root_case2();
        let mut mytree2 = MyStructTree {
            root: root2,
            children: Vec::new(),
        };

        mytree2.add(root1);

        let root1 = get_root_case1();
        let mut mytree3 = MyStructTree {
            root: root1,
            children: Vec::new(),
        };
        let expected_children = Vec::from([mytree3]);
        assert_eq!(mytree2.children(), &expected_children);
    }

    #[test]
    fn test_mystructtree_size_case1() {
        let root1 = get_root_case1();
        let mut mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        assert_eq!(mytree.size(), 1);
    }

    #[test]
    fn test_mystructtree_size_case2() {
        let root1 = get_root_case1();
        let mut mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        let root2 = get_root_case2();
        let mut mytree2 = MyStructTree {
            root: root2,
            children: Vec::new(),
        };

        mytree2.add_child(mytree);

        assert_eq!(mytree2.size(), 2);
    }

    #[test]
    fn test_mystructtree_remove_child_case1() {
        let root1 = get_root_case1();
        let mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        let root2 = get_root_case2();
        let mut mytree2 = MyStructTree {
            root: root2,
            children: Vec::new(),
        };

        mytree2.remove_child(&mytree);
        assert_eq!(mytree2.size(), 1);
    }

    #[test]
    fn test_mystructtree_remove_child_case2() {
        let root1 = get_root_case1();
        let mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        let root2 = get_root_case2();
        let mut mytree2 = MyStructTree {
            root: root2,
            children: Vec::from([mytree]),
        };

        assert_eq!(mytree2.size(), 2);

        let root1 = get_root_case1();
        let mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };
        mytree2.remove_child(&mytree);
        assert_eq!(mytree2.size(), 1);
    }

    #[test]
    fn test_mystructtree_remove_case1() {
        let root1 = get_root_case1();
        let mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        let root2 = get_root_case2();
        let mut mytree2 = MyStructTree {
            root: root2,
            children: Vec::new(),
        };

        let root1 = get_root_case1();
        mytree2.remove(&root1);
        assert_eq!(mytree2.size(), 1);
    }

    #[test]
    fn test_mystructtree_remove_case2() {
        let root1 = get_root_case1();
        let mytree = MyStructTree {
            root: root1,
            children: Vec::new(),
        };

        let root2 = get_root_case2();
        let mut mytree2 = MyStructTree {
            root: root2,
            children: Vec::from([mytree]),
        };

        assert_eq!(mytree2.size(), 2);
        let root1 = get_root_case1();
        mytree2.remove(&root1);
        assert_eq!(mytree2.size(), 1);
    }
}
