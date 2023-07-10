use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    // 询问用户输入源文件夹路径
    println!("请输入源文件夹路径：");
    let source_folder = input_path();

    // 询问用户输入目标文件夹路径
    println!("请输入目标文件夹路径：");
    let destination_folder = input_path();

    // 询问用户输入文件后缀名
    println!("请输入要移动的文件后缀名：");
    let file_extension = input_string();

    match move_files_with_extension(&source_folder, &destination_folder, &file_extension) {
        Ok(_) => println!("文件移动完成！"),
        Err(e) => eprintln!("文件移动出错：{}", e),
    }
}

fn input_path() -> PathBuf {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let path = input.trim();
    PathBuf::from(path)
}

fn input_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().to_string()
}

fn move_files_with_extension(
    source_folder: &Path,
    destination_folder: &Path,
    file_extension: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 创建目标文件夹
    fs::create_dir_all(destination_folder)?;

    // 遍历源文件夹及其子文件夹
    for entry in fs::read_dir(source_folder)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == file_extension {
                // 构建目标文件的路径
                let file_name = path.file_name().unwrap();
                let destination = destination_folder.join(file_name);

                // 复制文件
                fs::copy(&path, &destination)?;
                println!("移动文件：{:?} -> {:?}", path, destination);
            }
        }
        if path.is_dir() {
            // 递归调用 move_files_with_extension 处理子文件夹
            move_files_with_extension(&path, destination_folder, file_extension)?;
        }
    }

    Ok(())
}