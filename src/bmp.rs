use crate::huffman_tree::HuffmanTree;
use std::fs::File;
use std::io::{Read, Write};

pub struct BmpImage {
    pub header: Vec<u8>,
    pub data: Vec<u8>,
}

impl BmpImage {
    pub fn read(filename: &str) -> std::io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut header = vec![0; 54]; // 标准BMP头部大小
        file.read_exact(&mut header)?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        Ok(BmpImage { header, data })
    }

    pub fn write(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(&self.header)?;
        file.write_all(&self.data)?;
        Ok(())
    }

    pub fn compress(&self) -> std::io::Result<Vec<u8>> {
        let mut huffman_tree = HuffmanTree::new();
        huffman_tree.build(&self.data);

        // 压缩数据
        let compressed_data = huffman_tree.encode(&self.data);

        // 构建压缩文件格式
        let mut result = Vec::new();

        // 1. 写入原始数据大小（8字节）
        result.extend_from_slice(&(self.data.len() as u64).to_le_bytes());

        // 2. 写入哈夫曼树
        let tree_data = huffman_tree.serialize();
        result.extend_from_slice(&(tree_data.len() as u32).to_le_bytes());
        result.extend_from_slice(&tree_data);

        // 3. 写入BMP头部
        result.extend_from_slice(&self.header);

        // 4. 写入压缩后的数据
        result.extend_from_slice(&compressed_data);

        Ok(result)
    }

    pub fn decompress(compressed: &[u8]) -> std::io::Result<Self> {
        let mut offset = 0;

        // 1. 读取原始数据大小
        let original_size = u64::from_le_bytes(compressed[0..8].try_into().unwrap()) as usize;
        offset += 8;

        // 2. 读取哈夫曼树大小和数据
        let tree_size =
            u32::from_le_bytes(compressed[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        let tree_data = &compressed[offset..offset + tree_size];
        offset += tree_size;

        // 3. 读取BMP头部
        let header = compressed[offset..offset + 54].to_vec();
        offset += 54;

        // 4. 读取压缩数据
        let compressed_data = &compressed[offset..];

        // 5. 重建哈夫曼树并解压数据
        let huffman_tree = HuffmanTree::deserialize(tree_data).ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid tree data",
        ))?;

        let data = huffman_tree.decode(compressed_data, original_size);

        Ok(BmpImage { header, data })
    }
}
