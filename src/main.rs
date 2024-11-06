use std::fs::{File, OpenOptions};
use std::io::{self, Write, Seek, SeekFrom};
use rand::Rng;

// 定义覆盖次数
const OVERWRITE_TIMES: u32 = 35;

fn shred_file(path: &str) -> io::Result<()> {
    // 打开文件以进行读写操作
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)?;

    // 获取文件大小
    let file_size = file.metadata()?.len();

    // 开始覆盖文件内容
    for _ in 0..OVERWRITE_TIMES {
        // 重置文件指针到文件开头
        file.seek(SeekFrom::Start(0))?;

        // 随机生成覆盖数据
        let mut buffer = vec![0u8; file_size as usize];
        rand::thread_rng().fill(&mut buffer[..]);

        // 写入覆盖数据
        file.write_all(&buffer)?;
        file.flush()?; // 确保数据写入到磁盘
    }

    // 覆盖完成后，删除文件
    std::fs::remove_file(path)?;

    Ok(())
}

fn main() {
    let path = "D:/desktop/远程 - 副本.txt"; // 要粉碎的文件路径
    match shred_file(path) {
        Ok(_) => println!("文件已成功粉碎！"),
        Err(e) => eprintln!("粉碎文件时出错: {}", e),
    }
}
