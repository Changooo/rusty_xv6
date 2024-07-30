extern "C" {
    // !!why need static????
    // static mut ptable: proctable;
    // !!cons와 consputc를 cprint 정의에서 안쓰고 여기서 써야만하는가..
    // static mut cons: conslock;
    // fn consputc(c: i32);
    // fn acquire(lk: *mut spinlock);
    // fn release(lk: *mut spinlock);
}

#[macro_export]
macro_rules! cprint {
    ($fmt:expr) => {{
        let locking:i32 = cons.locking;
        if locking != 0 {
            acquire(&mut cons.lock);
        }
        let mut fmts = $fmt.chars();
        while let Some(c) = fmts.next() {
            consputc(c as i32);
        }
        if locking != 0 {
            release(&mut cons.lock);
        }
    }};
    ($fmt:expr $(, $arg:expr)*) => {{
        let locking:i32 = cons.locking;
        if locking != 0 {
            acquire(&mut cons.lock);
        }
        let mut fmts = $fmt.chars();
        // !!locking안에서 왜 heap을 할당할 수 없는가? 그리고 Vec<char>도 heap아닌가?
        let args = [$(&$arg as &dyn Printable),*];
        let mut args_iter = 0;

        while let Some(c) = fmts.next() {
            if c == '{' {
                if let Some('}') = fmts.clone().next() {
                    fmts.next(); // Consume the '}'
                    if args_iter < args.len() {
                        args[args_iter].printx();
                        args_iter += 1;
                    }
                } else {
                    consputc(c as i32);
                }
            } else {
                consputc(c as i32);
            }
        }
        if locking != 0 {
            release(&mut cons.lock);
        }
    }};
}
