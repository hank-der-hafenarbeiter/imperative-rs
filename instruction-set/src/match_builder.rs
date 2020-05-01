use std::collections::HashMap;
use crate::Instruction;

enum Node<'a>{
    Root{children:HashMap<&'a[char; 8], Node<'a>>},
    Inner{depth:usize, children:HashMap<&'a[char; 8], Node<'a>>},
    Leaf{depth: usize, instr:Instruction},
}

impl<'a> Node <'a> {
   fn new_root() -> Self {
        Self::Root{
            children:HashMap::new(),
        }
   }
    
   fn new_inner(depth:usize) -> Self {
        Self::Inner{
            depth,
            children:HashMap::new(),
        }
   }

   fn new_leaf(depth:usize, instr:Instruction) -> Self {
        Self::Leaf{
            depth,
            instr,
        }
   }


}

struct MatchBuilder<'a> {
    root:Node<'a>,
}

impl<'a> MatchBuilder<'a> {
    
    fn construct() -> MatchBuilder<'static> {
        MatchBuilder{
            root:Node::new_root(),
        }
    }

    fn insert(&self, instr:Instruction) -> Result<(), ()> {
        let mut cur_node = &self.root;
        Ok(())
    }
}
