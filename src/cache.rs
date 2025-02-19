use std::io::{self};
use std::path::Path;

use rusqlite::{Connection, Result};
// 1.初始化缓存系统
pub fn init_cache() -> Result<bool, io::Error> {
    let path = Path::new(".Cache");
    if path.exists() && path.is_dir() {
        println!("目录存在。");
    } else {
        println!("目录不存在。");
        match std::fs::create_dir_all(".Cache") {
            Ok(_) => {
                println!("创建目录成功");
            }
            Err(e) => {
                println!("创建目录失败: {:?}", e);
                return Err(e);
            }
        }
    }
    let path = Path::new(".Cache/brieface.db");
    if path.exists() {
        println!("数据库文件存在！");
    } else {
        println!("数据库文件不存在！");
        match std::fs::File::create(".Cache/brieface.db") {
            Ok(_) => {
                println!("创建数据库文件成功");
                match init_db() {
                    Ok(_) => println!("初始化数据库成功"),
                    Err(e) => {
                        println!("初始化数据库失败: {:?}", e);
                        // return Err(e);
                    }
                }
            }
            Err(e) => {
                println!("创建数据库文件失败: {:?}", e);
                return Err(e);
            }
        }
    }
    return Ok(true);
}

// 2.初始化数据库
pub fn init_db() -> Result<bool, rusqlite::Error> {
    let conn = Connection::open(".Cache/brieface.db")?;
    match conn.execute(
        "CREATE TABLE users (
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            name  TEXT NOT NULL,
            uuid  BLOB
        )",
        (),
    ) {
        Ok(_) => println!("创建user表成功"),
        Err(e) => {
            println!("创建user表失败: {:?}", e);
            return Err(e);
        }
    }
    return Ok(true);
}
