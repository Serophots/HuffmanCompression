pub struct HuffmanValue {
    pub(crate) char: char,
    pub(crate) count: usize
}

pub struct HuffmanBranch {
    pub(crate) t: Box<HuffmanNode>,
    pub(crate) f: Box<HuffmanNode>,
    pub(crate) count: usize,
}
impl HuffmanBranch {
    pub fn is_t_none(&self) -> bool {
        if let HuffmanNode::None = *self.t {
            return true;
        }
        false
    }

    pub fn is_f_none(&self) -> bool {
        if let HuffmanNode::None = *self.f {
            return true;
        }
        false
    }

}


pub enum HuffmanNode {
    Branch(Box<HuffmanBranch>),
    Value(HuffmanValue),
    None
}
impl HuffmanNode {
    pub fn count_sub_branches(&self) -> usize {
        match self {
            HuffmanNode::Branch(branch) => {
                branch.count
            },
            HuffmanNode::Value(value) => {
                value.count
            },
            _ => 0
        }
    }
}
