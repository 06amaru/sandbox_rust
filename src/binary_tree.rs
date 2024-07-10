use std::{fmt::Display, rc::Rc};
use std::cell::RefCell;
use charming::{ImageRenderer,element::{Emphasis, EmphasisFocus, Label, LabelAlign, LabelPosition, LabelVerticalAlign}, series::{Tree, TreeNode}, Chart};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct BinarySearchTree<T>{
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Ord + Clone + Serialize + Display> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree{root: None}
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        if let Some(root) = &self.root {
            Self::insert_recursive(root.clone(), new_node);
        } else {
            self.root = Some(new_node)
        }
    }

    fn insert_recursive(node: Rc<RefCell<Node<T>>>, new_node: Rc<RefCell<Node<T>>>) {
        let mut node = node.borrow_mut();
        if new_node.borrow().value < node.value {
            if let Some(left) = &node.left {
                Self::insert_recursive(left.clone(), new_node);
            } else {
                node.left = Some(new_node);
            }
        } else {
            if let Some(right) = &node.right {
                Self::insert_recursive(right.clone(), new_node);
            } else {
                node.right = Some(new_node);
            }
        }
    }

    pub fn search(&self, value: &T) -> bool {
        self.root.as_ref().map_or(false, |root| Self::search_recursive(root, value))
    }

    fn search_recursive(node: &Rc<RefCell<Node<T>>>, value: &T) -> bool {
        let node = node.borrow();
        if &node.value == value {
            true
        } else if value < &node.value {
            node.left.as_ref().map_or(false, |left| Self::search_recursive(left, value))
        } else {
            node.right.as_ref().map_or(false, |right| Self::search_recursive(right, value))
        }
    }

    pub fn delete(&mut self, value: T) {
        if let Some(root) = self.root.take() {
            let new_root = Self::delete_recursive(root, &value);
            self.root = new_root;
        }
    }

    fn delete_recursive(node: Rc<RefCell<Node<T>>>, value: &T) -> Option<Rc<RefCell<Node<T>>>> {
        let mut node_ref = node.borrow_mut();
        if value < &node_ref.value {
            if let Some(left) = node_ref.left.take() {
                node_ref.left = Self::delete_recursive(left, value);
            }
        } else if value > &node_ref.value {
            if let Some(right) = node_ref.right.take() {
                node_ref.right = Self::delete_recursive(right, value);
            }
        } else {
            if node_ref.left.is_none() {
                return node_ref.right.take();
            } else if node_ref.right.is_none() {
                return node_ref.left.take();
            }

            let min_right = Self::find_min(&node_ref.right.as_ref().unwrap());
            node_ref.value = min_right.borrow().value.clone();
            node_ref.right = Self::delete_recursive(node_ref.right.as_ref().unwrap().clone(), &node_ref.value);
        }

        drop(node_ref);
        Some(node.clone())
    }

    fn find_min(node: &Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        let node_ref = node.borrow();
        if let Some(left) = &node_ref.left {
            Self::find_min(left)
        } else {
            node.clone()
        }
    }

    pub fn display_graph(&self) {
        if let Some(root) = &self.root {
            let node_graph = Self::dfs(root);
            let json = serde_json::to_string(&node_graph).unwrap();
            let chart = Self::make_chart(&json);
            let _ = ImageRenderer::new(500, 400).save(&chart, "./binary_tree.svg");
        }
    }

    fn dfs(node: &Rc<RefCell<Node<T>>>) -> NodeGraph {
        let node_ref = node.borrow();
        let mut node_graph = NodeGraph::new(node_ref.value.clone().to_string());
        if let Some(left) = &node_ref.left {
            let new_node = Self::dfs(left);
            node_graph.children.push(new_node)
        }
        if let Some(right) = &node_ref.right {
            let new_node = Self::dfs(right);
            node_graph.children.push(new_node);
        }
        node_graph
    }

    fn make_chart(s: &str) -> Chart {
        let data: TreeNode = serde_json::from_str(s).unwrap();
        Chart::new()
        .series(
            Tree::new()
                .name("BINARY TREE")
                .top("1%")
                .left("7%")
                .bottom("1%")
                .right("20%")
                .symbol_size(7.0)
                .label(
                    Label::new()
                    .position(LabelPosition::Right)
                    .vertical_align(LabelVerticalAlign::Middle)
                    .align(LabelAlign::Left)
                    .font_size(9.0),)
                .emphasis(Emphasis::new().focus(EmphasisFocus::Descendant))
                .expand_and_collapse(false)
                .data(vec![data])
        )
    }

}

#[derive(Serialize, Deserialize)]
struct NodeGraph {
    name: String,
    children: Vec<NodeGraph>
}

impl NodeGraph {
    pub fn new(name: String) -> Self {
        NodeGraph {
            name: name,
            children: Vec::new(),
        }
    }
}



pub fn binary_tree_test() {
    let mut tree = BinarySearchTree::<i64>::new();
    tree.insert(40);
    tree.insert(50);
    tree.insert(30);
    tree.insert(10);
    tree.insert(70);
    tree.insert(80);
    tree.insert(60);
    tree.insert(45);
    tree.insert(49);
    tree.insert(41);
    tree.insert(5);
    tree.insert(20);
    tree.insert(35);
    tree.insert(33);
    tree.insert(39);
    tree.insert(44);
    tree.delete(40);

    tree.display_graph();
}
