use crate::huffman_node::HuffmanNode;
use std::collections::{BinaryHeap, HashMap};

pub struct HuffmanTree {
    pub root: Option<Box<HuffmanNode>>,
}

impl HuffmanTree {
    pub fn new() -> Self {
        HuffmanTree { root: None }
    }

    pub fn build(&mut self, data: &[u8]) {
        let frequencies = self.calculate_frequencies(data);
        let mut heap = self.create_priority_queue(frequencies);

        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();

            let parent = Box::new(HuffmanNode {
                frequency: left.frequency + right.frequency,
                value: None,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            });

            heap.push(*parent);
        }

        self.root = heap.pop().map(Box::new);
    }

    fn calculate_frequencies(&self, data: &[u8]) -> HashMap<u8, u32> {
        let mut frequencies = HashMap::new();
        for &byte in data {
            *frequencies.entry(byte).or_insert(0) += 1;
        }
        frequencies
    }

    fn create_priority_queue(&self, frequencies: HashMap<u8, u32>) -> BinaryHeap<HuffmanNode> {
        let mut heap = BinaryHeap::new();
        for (value, frequency) in frequencies {
            heap.push(HuffmanNode::new(Some(value), frequency));
        }
        heap
    }

    pub fn encode(&self, data: &[u8]) -> Vec<u8> {
        let codes = self.generate_codes();
        let mut encoded = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_count = 0;

        for &byte in data {
            if let Some(code) = codes.get(&byte) {
                for &bit in code {
                    current_byte = (current_byte << 1) | bit as u8;
                    bit_count += 1;

                    if bit_count == 8 {
                        encoded.push(current_byte);
                        current_byte = 0;
                        bit_count = 0;
                    }
                }
            }
        }

        if bit_count > 0 {
            current_byte <<= 8 - bit_count;
            encoded.push(current_byte);
        }

        encoded
    }

    fn generate_codes(&self) -> HashMap<u8, Vec<bool>> {
        let mut codes = HashMap::new();
        if let Some(root) = &self.root {
            self.generate_codes_recursive(root, &mut Vec::new(), &mut codes);
        }
        codes
    }

    fn generate_codes_recursive(
        &self,
        node: &Box<HuffmanNode>,
        current_code: &mut Vec<bool>,
        codes: &mut HashMap<u8, Vec<bool>>,
    ) {
        if let Some(value) = node.value {
            codes.insert(value, current_code.clone());
        } else {
            if let Some(left) = &node.left {
                current_code.push(false);
                self.generate_codes_recursive(left, current_code, codes);
                current_code.pop();
            }
            if let Some(right) = &node.right {
                current_code.push(true);
                self.generate_codes_recursive(right, current_code, codes);
                current_code.pop();
            }
        }
    }

    pub fn decode(&self, encoded: &[u8], original_size: usize) -> Vec<u8> {
        let mut decoded = Vec::with_capacity(original_size);
        if let Some(root) = &self.root {
            let mut current = root;

            for &byte in encoded {
                for bit_pos in (0..8).rev() {
                    let bit = (byte >> bit_pos) & 1 == 1;

                    current = if bit {
                        if let Some(right) = &current.right {
                            right
                        } else {
                            continue;
                        }
                    } else {
                        if let Some(left) = &current.left {
                            left
                        } else {
                            continue;
                        }
                    };

                    if let Some(value) = current.value {
                        decoded.push(value);
                        if decoded.len() == original_size {
                            return decoded;
                        }
                        current = root;
                    }
                }
            }
        }
        decoded
    }

    // 序列化哈夫曼树，用于保存树结构
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        if let Some(root) = &self.root {
            self.serialize_node(root, &mut result);
        }
        result
    }

    fn serialize_node(&self, node: &Box<HuffmanNode>, result: &mut Vec<u8>) {
        // 1 for inner node，0 for leaf node
        if node.value.is_some() {
            result.push(0);
            result.push(node.value.unwrap());
        } else {
            result.push(1);
            if let Some(left) = &node.left {
                self.serialize_node(left, result);
            }
            if let Some(right) = &node.right {
                self.serialize_node(right, result);
            }
        }
    }

    // 从序列化数据重建哈夫曼树
    pub fn deserialize(data: &[u8]) -> Option<Self> {
        let mut index = 0;
        let root = Self::deserialize_node(data, &mut index)?;
        Some(HuffmanTree {
            root: Some(Box::new(root)),
        })
    }

    fn deserialize_node(data: &[u8], index: &mut usize) -> Option<HuffmanNode> {
        if *index >= data.len() {
            return None;
        }

        let node_type = data[*index];
        *index += 1;

        if node_type == 0 {
            if *index >= data.len() {
                return None;
            }
            let value = data[*index];
            *index += 1;
            Some(HuffmanNode::new(Some(value), 0))
        } else {
            // 内部节点
            let left = Self::deserialize_node(data, index)?;
            let right = Self::deserialize_node(data, index)?;
            Some(HuffmanNode {
                frequency: 0,
                value: None,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            })
        }
    }
}
