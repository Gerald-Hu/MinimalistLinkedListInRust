#[derive(Debug)]
struct Node<T>{
    data : T,
    next : Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LinkedListStack<T>{
    head : Option<Box<Node<T>>>,
}


impl<T: std::fmt::Debug> LinkedListStack<T>{

    fn push(&mut self, item: T){
    
        let mut new_node: Box<Node<T>> = Box::new(Node{data:item, next: None});
        
        match self.head{
        
            None =>{
                self.head = Some(new_node);
            }
            _ =>{
                new_node.next = self.head.take();
                self.head = Some(new_node);
            }
        };
    }
    
    fn top(&self) -> Option<&T>{

        if let Some(node) = &self.head{
        
            Some(&node.data)    
            
        }else{
        
            None
            
        }
    }
    
    fn pop(&mut self){
    
        if let Some(node) = self.head.take(){
            self.head = node.next;
        }
        
    }
    
}


fn main() {
    let mut stack: LinkedListStack<&str> = LinkedListStack{head: None};


    stack.push("Sleepy");
    stack.push("So");
    stack.push("Me");

    println!("What's on the top: {:?}", stack.top());
    stack.pop();

    println!("What's on the top: {:?}", stack.top());
    stack.pop();

    println!("What's on the top: {:?}", stack.top());
    stack.pop();
    println!("What's on the top: {:?}", stack.top());
    
    println!("{:?}",stack);
}
