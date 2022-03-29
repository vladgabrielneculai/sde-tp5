use std::{io::stdin, os::unix::process::parent_id, thread,time};
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, write, Pid, getpid, getppid}};

fn main() {

    //exercice 1

    let mut string = String::new();
    stdin().read_line(& mut string).unwrap();
    let transf = string.as_str();
    let mut b = Box::new(String::from("text fix ").to_string());
    b.push_str(transf);
    println!("{}",b);

    //exercice 2

    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("parent with pid: {}, child with pid: {}", getppid(), child);
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            thread::sleep(time::Duration::from_secs(3));
            println!("I am the child with PID: {}, and my parent has PID: {}",getpid(),getppid());
        }
        Err(_) => println!("Fork failed"),
    }

}
