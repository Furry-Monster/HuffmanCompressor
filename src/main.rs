mod bmp;
mod huffman_node;
mod huffman_tree;

use crate::bmp::BmpImage;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Huffman压缩/解压工具");
        println!("----------------------");
        println!("用法:");
        println!(" -> 压缩: {} compress <input.bmp>", args[0]);
        println!(" -> 解压: {} decompress <input.bmp.compressed>", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "compress" => compress_command(&args[2])?,
        "decompress" => decompress_command(&args[2])?,
        _ => println!("未知命令: {}", args[1]),
    }

    Ok(())
}

fn compress_command(input_file: &str) -> std::io::Result<()> {
    let image = BmpImage::read(input_file)?;
    let original_size = image.data.len();

    let compressed = image.compress()?;
    let output_file = format!("{}.compressed", input_file);
    std::fs::write(&output_file, &compressed)?;

    println!("压缩完成！");
    println!("原始大小: {} 字节", original_size);
    println!("压缩后大小: {} 字节", compressed.len());
    println!(
        "压缩率: {:.2}%",
        (1.0 - (compressed.len() as f64 / original_size as f64)) * 100.0
    );

    Ok(())
}

fn decompress_command(input_file: &str) -> std::io::Result<()> {
    let compressed_data = std::fs::read(input_file)?;
    let image = BmpImage::decompress(&compressed_data)?;

    let output_file = input_file.replace(".compressed", ".decompressed.bmp");
    image.write(&output_file)?;

    println!("解压完成！");
    println!("解压后文件: {}", output_file);

    Ok(())
}
