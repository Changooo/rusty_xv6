const NSEGS:usize = 6;
const NOFILE:usize = 16;
const NDIRECT:usize = 12;
const PIPESIZE:usize = 512;
pub const NPROC:usize = 64;

type PdeT = i32;

#[repr(C)]
pub struct spinlock {
    locked: u32,         // Is the lock held?
    name: *mut u8,     // Name of lock.
    cpu: *mut cpu,       // The cpu holding the lock.
    pcs: [u32; 10],      // The call stack (an array of program counters) that locked the lock.
}

#[repr(C)]
pub struct segdesc {
    a: u32,
    b: u32,
    // uint lim_15_0 : 16;  // Low bits of segment limit
    // uint base_15_0 : 16; // Low bits of segment base address
    // uint base_23_16 : 8; // Middle bits of segment base address
    // uint type : 4;       // Segment type (see STS_ constants)
    // uint s : 1;          // 0 = system, 1 = application
    // uint dpl : 2;        // Descriptor Privilege Level
    // uint p : 1;          // Present
    // uint lim_19_16 : 4;  // High bits of segment limit
    // uint avl : 1;        // Unused (available for software use)
    // uint rsv1 : 1;       // Reserved
    // uint db : 1;         // 0 = 16-bit segment, 1 = 32-bit segment
    // uint g : 1;          // Granularity: limit scaled by 4K when set
    // uint base_31_24 : 8; // High bits of segment base address
}

#[repr(C)]
pub struct taskstate {
    link: u32,                          // Old ts selector
    esp0: u32,                          // Stack pointers and segment selectors
    ss0: u16,                           //   after an increase in privilege level
    padding1: u16,
    esp1: *mut u32,
    ss1: u16,
    padding2: u16,
    esp2: *mut u32,
    ss2: u16,
    padding3: u16,
    cr3: *mut std::ffi::c_void,         // Page directory base
    eip: *mut u32,                      // Saved state from last task switch
    eflags: u32,
    eax: u32,                           // More saved state (registers)
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: *mut u32,
    ebp: *mut u32,
    esi: u32,
    edi: u32,
    es: u16,                            // Even more saved state (segment selectors)
    padding4: u16,
    cs: u16,
    padding5: u16,
    ss: u16,
    padding6: u16,
    ds: u16,
    padding7: u16,
    fs: u16,
    padding8: u16,
    gs: u16,
    padding9: u16,
    ldt: u16,
    padding10: u16,
    t: u16,                             // Trap on task switch
    iomb: u16,                          // I/O map base address
}

#[repr(C)]
pub struct context{
    edi: u32,
    esi: u32,
    ebx: u32,
    ebp: u32,
    eip: u32,
}

#[repr(C)]
pub struct cpu {
    apicid: u8,                     // Local APIC ID
    scheduler: *mut context,        // swtch() here to enter scheduler
    ts: taskstate,                  // Used by x86 to find stack for interrupt
    gdt: [segdesc;NSEGS],                     // x86 global descriptor table
    started: u32,                   // Has the CPU started?
    ncli: i32,                      // Depth of pushcli nesting.
    intena: i32,                    // Were interrupts enabled before pushcli?
    proc: *mut proc,                // The process running on this cpu or null
} 

#[repr(C)]
pub struct trapframe {
    // registers as pushed by pusha
    edi: u32,
    esi: u32,
    ebp: u32,
    oesp: u32,      // useless & ignored
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
  
    // rest of trap frame
    gs: u16,
    padding1: u16,
    fs: u16,
    padding2: u16,
    es: u16,
    padding3: u16,
    ds: u16,
    padding4: u16,
    trapno: u32,
  
    // below here defined by x86 hardware
    err: u32,
    eip: u32,
    cs: u16,
    padding5: u16,
    eflags: u32,
  
    // below here only when crossing rings, such as from user to kernel
    esp: u32,
    ss: u16,
    padding6: u16,
}
#[repr(C)]
pub struct pipe {
    lock: spinlock,
    data: [u8; PIPESIZE],
    nread: u32,     // number of bytes read
    nwrite: u32,    // number of bytes written
    readopen: i32,   // read fd is still open
    writeopen: i32,  // write fd is still open
}
#[repr(C)]
pub enum FTYPE{ FdNONE, FdPIPE, FdINODE }

#[repr(C)]
pub struct file {
    type_: FTYPE,
    ref_: i32,            // reference count
    readable: u8,
    writable: u8,
    pipe: *mut pipe,
    ip: *mut inode,
    off: u32,
}

#[repr(C)]
pub struct sleeplock {
    locked: u32,        // Is the lock held?
    lk: spinlock,       // spinlock protecting this sleep lock

    // For debugging:
    name: *mut u8,    // Name of lock.
    pid: u32,           // Process holding lock
}
  
// in-memory copy of an inode
#[repr(C)]
pub struct inode {
    dev: u32,                   // Device number
    inum: u32,                  // Inode number
    ref_: i32,                  // Reference count
    lock: sleeplock,            // protects everything below here
    valid: i32,                 // inode has been read from disk?

    type_: i16,                 // copy of disk inode
    major: i16,
    minor: i16,
    nlink: i16,
    size: u32,
    addrs: [u32; NDIRECT+1],
}

#[repr(C)]
pub enum procstate { UNUSED, EMBRYO, SLEEPING, RUNNABLE, RUNNING, ZOMBIE }
// pub enum procstate { UNUSED("UNUSED"), EMBRYO("EMBRYO"), SLEEPING("SLEEPING"), RUNNABLE("RUNNABLE"), RUNNING("RUNNING"), ZOMBIE("ZOMBIE") }

#[repr(C)]
pub struct proc {
    sz: u32,                            // Size of process memory (bytes)
    pgdir: *mut PdeT,                  // Page table
    kstack: *mut u8,                  // Bottom of kernel stack for this process
    pub state: procstate,                   // Process state
    pub pid: i32,                           // Process ID
    parent: *mut proc,                  // Parent process
    tf: *mut trapframe,                 // Trap frame for current syscall
    context: *mut context,              // swtch() here to run process
    chan: *mut std::ffi::c_void,        // If non-zero, sleeping on chan
    killed: i32,                        // If non-zero, have been killed
    ofile: [*mut file; NOFILE],      // Open files
    cwd: *mut inode,                 // Current directory
    pub name: [u8; 16],                   // Process name (debugging)
    pub nice: i32,
}


#[repr(C)]
pub struct proctable{
    pub lock: spinlock,
    pub proc: [proc; NPROC],
}

#[repr(C)]
pub struct conslock{
    pub lock: spinlock,
    pub locking: i32,
}