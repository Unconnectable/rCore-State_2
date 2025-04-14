/* use std::thread;
use std::time::Duration; */
#[allow(warnings)] // 忽略整个文件的警告
#[allow(unused_variables)]
#[allow(unused_imports)]
fn main() {
    /* println!("\x1b[31m 子线程{} \x1b[0m" );

    
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("\x1b[31m 子线程{} \x1b[0m",i);
            //println!("{}子线程", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread::spawn(|| {
        for i in 1..10 {
            //println!("hi number {} from the spawned thread!", i);
            //thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..10 {
        println!("\x1b[32m 主线程{} \x1b[0m",i);
        //println!("  {}主线程", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); */

    let mut x = 0;

    let mut y = &mut x;
    *y = 1;
    println!("{}", *y);

    let mut z = &mut x;
    *z = 2;
    println!("{}", *z);
}
