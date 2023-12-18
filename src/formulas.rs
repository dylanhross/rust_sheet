/*
    Module for evaluating formulas
*/


use crate::dtypes;


#[derive(Debug)]
pub struct TknTree {
    pub root: dtypes::TknLink,
}


impl TknTree {
    fn new() -> Self {
        TknTree { root: None}
    }

    fn push(&mut self, token: dtypes::FormToken) {
        let new_node = Box::new(dtypes::TknNode {
            token,
            left: self.root.take(),
            right: None
        });
        self.root = dtypes::TknLink::Some(new_node);
    }
}

pub fn tokens_to_tree (tokens: &mut Vec<dtypes::FormToken>) -> TknTree {
    let mut tree = TknTree::new();
    let mut token_it = tokens.iter_mut();
    // push the first token to the tree
    tree.push(token_it.next().unwrap().clone());
    while let Some(token) = token_it.next() {
        match token {
            dtypes::FormToken::BinOp(_op) => {
                // push the binop then get the next token and set the right side to that
                tree.push(token.clone());
                let new_node = Box::new(dtypes::TknNode {
                    token: token_it.next().unwrap().clone(),
                    left: None,
                    right: None,
                });
                let root = tree.root.as_mut().unwrap();
                root.right = dtypes::TknLink::Some(new_node);
            },
            _ => tree.push(token.clone())
        } 
    }
    tree
}


/* 
#[cfg(test)]
mod tests {
    use super::*;
    

    
}
*/
