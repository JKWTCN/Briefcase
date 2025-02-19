mod tools;
use tools::get_all_files;
mod cache;
use cache::init_cache;
mod summary;
use summary::md5;
use summary::sha1;

// fn main() {
//     let now_path = "briefcase";
//     match init_cache() {
//         Ok(_) => println!("初始化缓存成功"),
//         Err(e) => {
//             println!("初始化缓存失败: {:?}", e);
//             return;
//         }
//     }
//     let log_arr = get_all_files("briefcase");
//     println!("{:?}", log_arr);
//     for log in log_arr {
//         let now_file_path = format!("{}/{}", now_path, log);
//         println!("md5: {:?}", md5(now_file_path.clone()).unwrap());
//         println!("sha1: {:?}", sha1(now_file_path.clone()).unwrap());
//     }

//     // loop {}
// }

use clap::{App, Arg};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    // 解析命令行参数
    let matches = App::new("文件同步工具")
        .version("1.0")
        .author("iClouWar")
        .about("跨平台文件/目录同步工具")
        .arg(
            Arg::with_name("source")
                .help("源目录")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("target")
                .help("目标目录")
                .required(true)
                .index(2),
        )
        .get_matches();

    let source = Path::new(matches.value_of("source").unwrap());
    let target = Path::new(matches.value_of("target").unwrap());

    // 遍历源目录
    for entry in WalkDir::new(source) {
        let entry = entry?;
        let source_path = entry.path();

        // 构建目标路径
        let relative_path = source_path.strip_prefix(source).unwrap();
        let target_path = target.join(relative_path);

        // 处理目录
        if entry.file_type().is_dir() {
            if !target_path.exists() {
                println!("创建目录: {}", target_path.display());
                fs::create_dir_all(&target_path)?;
            }
            continue;
        }

        // 处理文件
        if entry.file_type().is_file() {
            sync_file(source_path, &target_path)?;
        }
    }

    Ok(())
}

fn sync_file(source: &Path, target: &Path) -> io::Result<()> {
    // 获取源文件元数据
    let source_metadata = fs::metadata(source)?;
    let source_mtime = source_metadata.modified()?;
    // 判断是否需要同步
    let need_sync = match fs::metadata(target) {
        Ok(target_metadata) => {
            // 判断时间戳是否一致
            let target_mtime = target_metadata.modified()?;
            // 创建时间一致判断文件内容是否一致
            if source_mtime == target_mtime {
                sha1(source).unwrap() != sha1(target).unwrap()
            } else {
                true
            }
        }
        Err(_) => true, // 目标文件不存在需要同步
    };

    if need_sync {
        // 确保目标目录存在
        if let Some(parent) = target.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        // 执行文件复制
        print!("同步文件: {} -> {}...", source.display(), target.display());
        io::stdout().flush()?;

        fs::copy(source, target)?;
        println!("完成");
    }

    Ok(())
}
