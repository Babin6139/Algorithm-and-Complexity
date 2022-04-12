use std::cmp::Ordering;
use std::fmt::Debug;
fn setup() -> BinarySearchTree {
    let mut bst: BinarySearchTree = BinarySearchTree::new();
    bst.add(10, "Value for 10");
    bst.add(52, "Value for 52");
    bst.add(5, "Value for 5");
    bst.add(8, "Value for 8");
    bst.add(1, "Value for 1");
    bst.add(40, "Value for 40");
    bst.add(30, "Value for 30");
    bst.add(45, "Value for 45");
    bst
}
#[cfg(test)]
mod Test {
    use super::*;
    #[test]
    fn test_add() {
        let mut bsTree = BinarySearchTree::new();
        assert_eq!(bsTree.size(), 0);

        bsTree.add(15, "Value for 15");
        assert_eq!(bsTree.size(), 1);

        bsTree.add(10, "Value for 10");
        assert_eq!(bsTree.size(), 2);
    }

    #[test]
    fn test_inorder() {
        let mut bst = setup();
        let actual_output = bst.inorder_walk();
        let expected_output = vec![1, 5, 8, 10, 30, 40, 45, 52];
        assert_eq!(actual_output, expected_output);

        bst.add(25, "Value for 25");
        assert_eq!(bst.inorder_walk(), vec![1, 5, 8, 10, 25, 30, 40, 45, 52]);
    }

    #[test]
    fn test_postorder() {
        let mut bst = setup();

        let actual_output = bst.postorder_walk();
        let expected_output = vec![1, 8, 5, 30, 45, 40, 52, 10];
        assert_eq!(actual_output, expected_output);

        bst.add(25, "Value for 25");
        assert_eq!(bst.postorder_walk(), vec![1, 8, 5, 25, 30, 45, 40, 52, 10]);
    }
    #[test]
    fn test_preorder() {
        let mut bst = setup();

        let actual_output = bst.preorder_walk();
        let expected_output = vec![10, 5, 1, 8, 52, 40, 30, 45];
        assert_eq!(actual_output, expected_output);

        bst.add(25, "Value for 25");
        assert_eq!(bst.preorder_walk(), vec![10, 5, 1, 8, 52, 40, 30, 25, 45]);
    }

    #[test]
    fn test_search() {
        let mut bst = setup();
        let actual_output = bst.search(40);
        let expected_output = MultiResponse::ResponseString(String::from("Value for 40"));
        assert_eq!(actual_output, expected_output);

        //For false return
        assert_eq!(bst.search(90), MultiResponse::ResponseBoolean(false));

        bst.add(90, "Value for 90");
        assert_eq!(
            bst.search(90),
            MultiResponse::ResponseString(String::from("Value for 90"))
        );
    }

    #[test]
    fn test_remove() {
        let mut bst = setup();
        bst.remove(40);
        assert_eq!(bst.size(), 7);
        assert_eq!(bst.inorder_walk(), vec![1, 5, 8, 10, 30, 45, 52]);
        assert_eq!(bst.preorder_walk(), vec![10, 5, 1, 8, 52, 30, 45]);
    }

    #[test]
    fn test_smallest() {
        let mut bst = setup();

        assert_eq!(bst.smallest(), (1, String::from("Value for 1")));
        bst.add(6, "Value for 6");
        bst.add(4, "Value for 4");
        bst.add(0, "Value for 0");
        bst.add(32, "Value for 32");

        assert_eq!(bst.smallest(), (0, String::from("Value for 0")));
    }
    #[test]

    fn test_largest() {
        let mut bst = setup();

        assert_eq!(bst.largest(), (52, String::from("Value for 52")));

        bst.add(6, "Value for 6");
        bst.add(54, "Value for 54");
        bst.add(0, "Value for 0");
        bst.add(32, "Value for 32");
        assert_eq!(bst.largest(), (54, String::from("Value for 54")));
    }
    #[test]
    fn test_nothing_to_remove() {
        let mut bsTree = BinarySearchTree::new();
        bsTree.add(10, "Value for 10");
        bsTree.remove(10);

        //Returns false if noting is removed or there is nothing to remove
        assert_eq!(bsTree.remove(10), false);
    }
    #[test]
    fn test_no_size_decrement() {
        let mut bsTree = BinarySearchTree::new();

        bsTree.add(10, "Value for 10");
        assert_eq!(bsTree.size(), 1);
        //Size must be decreased if there is node with that key present
        bsTree.remove(10);
        assert_eq!(bsTree.size(), 0);

        //Size must not decrease if there is no node with that key present
        assert_eq!(bsTree.remove(10), false);
        assert_eq!(bsTree.size(), 0);
    }

    #[test]
    fn test_no_nodes() {
        let mut bsTree = BinarySearchTree::new();

        //testing largest
        assert_eq!(bsTree.largest(), (-1, String::from("No items")));
        //testing smallest
        assert_eq!(bsTree.smallest(), (-1, String::from("No items")));

        //testing inorder traversal
        assert_eq!(bsTree.inorder_walk(), []);
        //testing preorder traversal
        assert_eq!(bsTree.preorder_walk(), []);

        //testing postorder traversal
        assert_eq!(bsTree.postorder_walk(), []);
    }
}

//structure named BinarySearchTree was created
#[derive(Debug, PartialEq)]
pub struct BinarySearchTree {
    size: i32,
    root: Option<Box<Node>>,
}

//structure named Node was created to represent the node of BinarySearchTree
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    key: i32,
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

//Some function needs multiple return type so we used enum for multiple return type from same function
#[derive(Debug, PartialEq)]
pub enum MultiResponse {
    ResponseString(String),
    ResponseBoolean(bool),
}

//Methods are defined within the context of structure BinarySearchTree
impl BinarySearchTree {
    //returns Self i.e BinarySearchTree with initial value of size 0 and root None
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }

    //Function to add new node in BST
    pub fn add(&mut self, key: i32, value: &str) {
        //size is increased on every insertion/addition of node
        self.size += 1;

        //New node intitialized with the key and value
        let new_node = Box::new(Node {
            key,
            //value is converted to string since value was obtained as string slices
            value: value.to_string(),
            left: None,
            right: None,
        });

        //If there is no root  then the new node is set as root
        if self.root == None {
            self.root = Some(new_node);
        } else {
            //else since root is of type Option<Box<Node>> so we match to get val of type Box<Node>
            match &mut self.root {
                Some(val) => Self::add_node(new_node, val), //obtained val is passed to method add_node along with new_node
                None => (), //IF there is None in root then it is executed
            }
        }
    }

    //It is called from add function to add a new node in the BST
    fn add_node(new_node: Box<Node>, current_node: &mut Box<Node>) {
        //reference of keys of the nodes were taken
        let ref new_val = new_node.key;
        let ref current_val = current_node.key;

        //checking if cuurent_val is greater than new_val
        if *current_val > *new_val {
            //checks if curent_node->left is Some or not
            if let Some(ref mut left) = current_node.left {
                Self::add_node(new_node, left); //it recursively calls itself with new_node and left
            } else {
                //if current_node->left=None then current_node->left is set to new node
                current_node.left = Some(new_node);
            }
        }
        //checking if cuurent_val is less than or equal to new_val
        else if *current_val <= *new_val {
            //checks if curent_node->right is Some or not
            if let Some(ref mut right) = current_node.right {
                Self::add_node(new_node, right); //it calls itself with new_node and right
            } else {
                //if current_node->right=None then current_node->right is set to new node
                current_node.right = Some(new_node);
            }
        }
    }

    //Returns the number of nodes in the BST
    pub fn size(&self) -> i32 {
        self.size
    }

    //returns the smallest key with its respective value in form of tuple
    pub fn smallest(&self) -> (i32, String) {
        //matching the value of self.root and Some value is present that val is passed as reference to smallest_r
        match &self.root {
            Some(val) => Self::smallest_r(&val),
            None => (-1, String::from("No items")), //For None condition the shown tuple is returned
        }
    }
    fn smallest_r(node: &Box<Node>) -> (i32, String) {
        //if node->left is None then the key and value of the node is returned as tuple
        if node.left == None {
            (node.key, node.value.clone())
        } else {
            if let Some(left) = &node.left {
                Self::smallest_r(&left) //Since node->left is Some so it calls itself by passing left
            } else {
                (-1, String::from("No items"))
            }
        }
    }

    //returns the largest key with its respective value in form of tuple
    pub fn largest(&self) -> (i32, String) {
        //matching the value of self.root and Some value is present that val is passed as reference to largest_r
        match &self.root {
            Some(val) => Self::largest_r(&val),
            None => (-1, String::from("No items")), //For None condition the shown tuple is returned
        }
    }
    fn largest_r(node: &Box<Node>) -> (i32, String) {
        //if node->right is None then the key and value of the node is returned as tuple
        if node.right == None {
            (node.key, node.value.clone())
        } else {
            if let Some(right) = &node.right {
                Self::largest_r(&right) //Since node->right is Some so it calls itself by passing right
            } else {
                (-1, String::from("No items"))
            }
        }
    }

    //this function returns either String or boolean so we used enum MultiResponse to return different data types
    //function to search key
    pub fn search(&self, key: i32) -> MultiResponse {
        match &self.root {
            Some(val) => Self::search_r(&val, key),
            None => MultiResponse::ResponseBoolean(false),
        }
    }

    //search function call this function to get either the value if key found or false if node with that key not found
    fn search_r(node: &Box<Node>, key: i32) -> MultiResponse {
        let ref current_val = node.key;
        //if root key matches the provided key return the value of the node
        if *current_val == key {
            MultiResponse::ResponseString(node.value.clone())
        } else if *current_val > key {
            if let Some(ref left) = node.left {
                Self::search_r(left, key) //provided key is less than node key so it call itself by passing left
            } else {
                MultiResponse::ResponseBoolean(false)
            }
        } else if *current_val < key {
            if let Some(ref right) = node.right {
                Self::search_r(right, key) //provided key is greater than node key so it call itself by passing right
            } else {
                MultiResponse::ResponseBoolean(false)
            }
        } else {
            MultiResponse::ResponseBoolean(false)
        }
    }

    //function to call to remove the node with certain key
    pub fn remove(&mut self, key: i32) -> bool {
        let result = Self::remove_r(&mut self.root, key);
        //if result is true i.e node with key is removed then the size is decreased by 1
        if result {
            self.size -= 1;
        }
        result //resullt is returned since it is an expression since it does not end with ;
    }

    //function called by remove to remove the node with the given key
    fn remove_r(mut node: &mut Option<Box<Node>>, key: i32) -> bool {
        if let Some(ref mut node1) = node {
            //the provided key is compared with the key of the node and according to the ordering some task is performed
            match node1.key.cmp(&key) {
                Ordering::Greater => Self::remove_r(&mut node1.left, key),
                Ordering::Less => Self::remove_r(&mut node1.right, key),
                Ordering::Equal => match (node1.left.as_mut(), node1.right.as_mut()) {
                    //checking if the node to delete has child or not
                    //it has no children
                    (None, None) => {
                        *node = None;
                        true
                    }

                    //it has left children and no right children
                    (Some(_), None) => {
                        let mut val: &mut Option<Box<Node>> = &mut node1.left;
                        *node = val.take();
                        true
                    }
                    //It has no left children but has right children
                    (None, Some(_)) => {
                        let mut val: &mut Option<Box<Node>> = &mut node1.right;
                        *node = val.take();
                        true
                    }

                    //It has both left children and right children
                    (Some(_), Some(_)) => {
                        //Value and key of the node to be deleted is replaced by the value and key of the largest node of node->left
                        let (s_key, value) = match &mut node1.left {
                            Some(value2) => Self::largest_r(&value2),
                            None => (-1, String::from("")),
                        };
                        //Removing the largest node of node->left
                        Self::remove_r(&mut node1.left, s_key);
                        node1.key = s_key;
                        node1.value = value;
                        true
                    }
                },
            }
        } else {
            false //if there is no node to delete it returns false
        }
    }

    //function called by inorder_walk and it returns list of node in inorder traversel
    fn inorder_traverse(node: &Option<Box<Node>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if *node == None {
            result //returns vec![] if there is no node
        } else {
            //perform inorder traversel (Left, Root, Right)
            if let Some(node1) = node {
                let mut a: Vec<i32> = Self::inorder_traverse(&node1.left);
                result.append(&mut a);
                result.push(node1.key);
                let mut a: Vec<i32> = Self::inorder_traverse(&node1.right);
                result.append(&mut a);
                result
            } else {
                result
            }
        }
    }

    //function called for inorder_walk
    pub fn inorder_walk(&self) -> Vec<i32> {
        Self::inorder_traverse(&self.root)
    }

    //function called by preorder_walk and it returns list of node in preorder traversel
    fn preorder_traverse(node: &Option<Box<Node>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if *node == None {
            result //returns vec![] if there is no node
        } else {
            //Perform preorder traversel (Root, Left, Right)
            if let Some(node1) = node {
                result.push(node1.key);
                let mut a: Vec<i32> = Self::preorder_traverse(&node1.left);
                result.append(&mut a);
                let mut a: Vec<i32> = Self::preorder_traverse(&node1.right);
                result.append(&mut a);
                result
            } else {
                result
            }
        }
    }

    //function called for preorder_walk
    pub fn preorder_walk(&self) -> Vec<i32> {
        Self::preorder_traverse(&self.root)
    }

    //function called by postorder_walk and it returns list of node in postorder traversel
    fn postorder_traverse(node: &Option<Box<Node>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if *node == None {
            result //returns vec![] if there is no node
        } else {
            //Perform preorder traversel (Root, Left, Right)
            if let Some(node1) = node {
                let mut a: Vec<i32> = Self::postorder_traverse(&node1.left);
                result.append(&mut a);
                let mut a: Vec<i32> = Self::postorder_traverse(&node1.right);
                result.append(&mut a);
                result.push(node1.key);
                result
            } else {
                result
            }
        }
    }

    //function called for postorder_walk
    pub fn postorder_walk(&self) -> Vec<i32> {
        Self::postorder_traverse(&self.root)
    }
}
