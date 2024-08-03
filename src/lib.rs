pub mod sequence_alignment;
pub mod benchmark;

pub trait StringUtils{
    fn equals(&self, other: &str) -> bool;
}
impl StringUtils for str{
    fn equals(&self, other: &str) -> bool {
        self == other
    }
}

pub mod linkedlist{
    #[derive(Debug, Clone)]
    pub struct LinkedList{
        pub head: Link<Node>,
    }
    #[derive(Debug, Clone)]
    pub struct Node{
        pub data: char,
        pub next: Link<Node>,
    }
    // impl Clone for Node {
    //     fn clone(&self) -> Node {
    //         Node {
    //             data: self.data,
    //             next: self.next.clone(),
    //         }
    //     }
    // }
    pub type Link<Node> = Option<Box<Node>>;
    impl LinkedList{
        pub fn new() -> Self{
            LinkedList{head: None}
        }
        pub fn pop(&mut self) -> Option<char>{
            match self.head.clone().take(){
                Some(n) => {
                    self.head = n.next;
                    Some(n.data)
                },
                None => None
            }
        }
        pub fn push(&mut self, data: char){
            let old_head = self.head.take();
            let new_head = Box::new(
                Node{data, next: old_head},);
            self.head = Some(new_head);
        }
        pub fn print(&self){
            let mut current = self.head.clone();
            while let Some(item) = current.take() {
                println!("{} ", item.data);
                current = item.next;
            }
        }
    }
    impl Node{
        pub fn new(data: char) -> Self{
            Self{
                data,
                next: None,
            }
        }

    }




}

