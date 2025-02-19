mod tools;
use tools::get_all_files;
mod cache;
use cache::init_cache;
mod summary;
use summary::md5;
use summary::sha1;

fn main() {
    let now_path = "briefcase";
    match init_cache() {
        Ok(_) => println!("初始化缓存成功"),
        Err(e) => {
            println!("初始化缓存失败: {:?}", e);
            return;
        }
    }
    let log_arr = get_all_files("briefcase");
    println!("{:?}", log_arr);
    for log in log_arr {
        let now_file_path = format!("{}/{}", now_path, log);
        println!("md5: {:?}", md5(now_file_path.clone()).unwrap());
        println!("sha1: {:?}", sha1(now_file_path.clone()).unwrap());
    }

    // loop {}
}
