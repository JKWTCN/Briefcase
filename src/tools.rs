// 获取目录内所有文件
pub fn get_all_files(dir: &str) -> Vec<String> {
    let paths = std::fs::read_dir(dir).unwrap(); // 可迭代对象ReadDir

    return paths
        .map(|f| {
            f.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<String>>();
}
