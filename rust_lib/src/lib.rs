
use crate::defi::spinlock;
use crate::defi::proctable;
use crate::defi::proc;
use crate::defi::procstate;
use crate::defi::conslock;
use crate::defi::NPROC;

//매크로는 use하지 않아도 사용할 수 있다. 
// use crate::defi::cprint;

//modoule 구조 명시
pub mod defi;
pub mod console;

// C 함수를 선언합니다.
extern "C" {
    // !!why need static????
    static mut ptable: proctable;
    // !!cons와 consputc를 cprint 정의에서 안쓰고 여기서 써야만하는가..
    static mut cons: conslock;
    fn consputc(c: i32);
    fn acquire(lk: *mut spinlock);
    fn release(lk: *mut spinlock);
    fn printint(xx: i32, base: i32, sign: i32);
    fn intlen(n: i32)->i32;
    fn padding(n: i32);
}


// 러스트에서 C 함수를 호출합니다.
#[no_mangle]
pub extern "C" fn ps(pid: i32){
    unsafe {
        let proc_array:&[proc; NPROC] = &ptable.proc;
        
        // !!enum출력가능? 그러면 enum으로 출력대체
        let strings:[&str; 6] = [ "UNUSED", "EMBRYO", "SLEEPING", "RUNNABLE", "RUNNING", "ZOMBIE" ];
        if pid==0 {
            cprint!("{}       {}        {}           {}       \n", "name", "pid", "state", "priority");
        }
        acquire(&mut ptable.lock);
        for p in proc_array{
            if pid==0 {
                match p.state {
                    procstate::UNUSED => {},
                    _ => {
                        cprint!("{} ", p.name);
                        padding(10-chararrlen(p.name));
                        cprint!("{} ", p.pid);
                        padding(10-intlen(p.pid));
                        let (state_str, str_cnt) = match p.state{
                            procstate::EMBRYO   => ("EMBRYO",   6),
                            procstate::SLEEPING => ("SLEEPING", 8),
                            procstate::RUNNABLE => ("RUNNABLE", 8),
                            procstate::RUNNING  => ("RUNNING",  7),
                            procstate::ZOMBIE   => ("ZOMBIE",   6),
                            _                   => ("GHOST",    5),
                        };
                        cprint!("{} ", state_str);
                        padding(15-str_cnt);
                        cprint!("{} ", p.nice as i32);
                        padding(15-intlen(p.nice));
                        cprint!("\n");
                    }
                }
            } 
            else if pid!=0 && p.pid == pid {
                match p.state {
                    procstate::UNUSED => {},
                    _ => {
                        cprint!("{} ", p.name);
                        padding(10-chararrlen(p.name));
                        cprint!("{} ", p.pid);
                        padding(10-intlen(p.pid));
                        let (state_str, str_cnt) = match p.state{
                            procstate::EMBRYO   => ("EMBRYO",   6),
                            procstate::SLEEPING => ("SLEEPING", 8),
                            procstate::RUNNABLE => ("RUNNABLE", 8),
                            procstate::RUNNING  => ("RUNNING",  7),
                            procstate::ZOMBIE   => ("ZOMBIE",   6),
                            _                   => ("GHOST",    5),
                        };
                        cprint!("{} ", state_str);
                        padding(15-str_cnt);
                        cprint!("{} ", p.nice as i32);
                        padding(15-intlen(p.nice));
                        cprint!("\n");
                    }
                }
            }
        }
        release(&mut ptable.lock);
    }
}

fn chararrlen(arr:[u8; 16])->i32{
    let mut count = 0;
    for c in arr{
        let c_to_usize = c as usize;
        if c_to_usize == 0 {
            return count;
        }
        count += 1;
    }
    -1
}


trait Printable {
    fn printx(&self);
}
impl Printable for i32 {
    fn printx(&self) {
        unsafe{
            printint(*self, 10, 0);
        }
    }
}

// !!이거 panic 이유
// for i in (0..count).rev() {
//     let asssss:i32 = digits[i]+48;
//     unsafe{
//         consputc(asssss);
//     }
// }

impl Printable for &str {
    fn printx(&self) {
        for c in self.chars() {
            unsafe{
                consputc(c as i32);            
            }
        }
    }
}

impl Printable for [u8;16] {
    fn printx(&self) {
        for c in self {
            if *c == 0 {
                break;
            }
            unsafe{
                consputc(*c as i32);            
            }
        }
    }
}