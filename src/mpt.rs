use crate::types::Hash;

pub type Nibble = u8;
pub type NodeRef = Hash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MptNode {
    Branch {
        children: [Option<NodeRef>; 16],
        value: Option<Vec<u8>>,
    },
    Leaf {
        path: Vec<Nibble>,
        value: Vec<u8>,
    },
    Extension {
        path: Vec<Nibble>,
        child: NodeRef,
    },
}

impl MptNode {
    pub fn branch() -> Self {
        Self::Branch { 
            children: [None; 16],
            value: None,
        }
    }

    pub fn branch_with_value(value: Vec<u8>) -> Self {
        Self::Branch { 
            children: [None; 16], 
            value: Some(value) 
        }
    }

    pub fn leaf(path: Vec<Nibble>, value: Vec<u8>) -> Self {
        Self::Leaf { path, value }
    }

    pub fn extension(path: Vec<Nibble>, child: NodeRef) -> Self {
        Self::Extension { path, child}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_starts_empty() {
        let node = MptNode::branch();

        let MptNode::Branch { children, value } = node else {
            panic!("expected branch node");
        };

        assert!(children.iter().all(Option::is_none));
        assert_eq!(value, None);
    }

    #[test]
    fn branch_can_store_value_for_key_that_ends_at_bench() {
        let node = MptNode::branch_with_value(b"account".to_vec());

        let MptNode::Branch { children, value } = node else {
            panic!("expected branch node");
        };

        assert!(children.iter().all(Option::is_none));
        assert_eq!(value, Some(b"account".to_vec()));
    }

    #[test]
    fn leaf_stores_remaining_path_and_value() {
        let node = MptNode::leaf(vec![0x0a, 0x0b, 0x0c], b"value".to_vec());

        assert_eq!(
            node,
            MptNode::Leaf {
                path: vec![0x0a, 0x0b, 0x0c],
                value: b"value".to_vec(),
            }
        );
    }

    #[test]
    fn extension_stores_shared_path_and_child_reference() {
        let child = [0x11u8; 32];
        let node = MptNode::extension(vec![0x01, 0x02], child);

        assert_eq!(
            node,
            MptNode::Extension {
                path: vec![0x01, 0x02],
                child,
            }
        );
    }
}