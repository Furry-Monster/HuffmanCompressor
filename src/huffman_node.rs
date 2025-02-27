#[derive(Debug, Eq, PartialEq)]
pub struct HuffmanNode {
    pub frequency: u32,
    pub value: Option<u8>,
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    pub fn new(value: Option<u8>, frequency: u32) -> Self {
        HuffmanNode {
            frequency,
            value,
            left: None,
            right: None,
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
