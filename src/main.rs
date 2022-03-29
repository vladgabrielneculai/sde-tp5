use std::{io::stdin, os::unix::process::parent_id, thread,time, ffi::CString, fs::File};
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, write, Pid, getpid, getppid, execvp, dup2}};
use std::os::unix::io::AsRawFd;

fn main() {

    //exercice 1

    let mut string = String::new();
    stdin().read_line(& mut string).unwrap();
    let transf = string.as_str();
    let mut b = Box::new(String::from("text fix ").to_string());
    b.push_str(transf);
    println!("{}",b);

    //exercice 2 & 3

    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("parent with pid: {}, child with pid: {}", getppid(), child);
            println!("En attente de la sortie du processus enfant");
            let status = waitpid(child, None).unwrap();
            println!("Done with status {:?}",status);
        }
        Ok(ForkResult::Child) => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            // thread::sleep(time::Duration::from_secs(3));
            println!("I am the child with PID: {}, and my parent has PID: {}",getpid(),getppid());
            // std::process::exit(30);
            let ls = CString::new("ls").unwrap();
            let l = CString::new("-l").unwrap();
            execvp(&ls,&[ls.clone(), l]);

            let fisier = File::create("output.txt").as_raw_fd();
            dup2(fisier,1);
        }
        Err(_) => println!("Fork failed"),
    }


}
