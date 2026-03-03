use int_enum::IntEnum;
use pod::Pod;

use crate::time::TimeVal;

#[repr(u32)]
#[derive(Eq, PartialEq, Debug, Copy, Clone, IntEnum)]
pub enum SyslogAction {
    CLOSE = 0,
    OPEN = 1,
    READ = 2,
    ReadAll = 3,
    ReadClear = 4,
    CLEAR = 5,
    ConsoleOff = 6,
    ConsoleOn = 7,
    ConsoleLevel = 8,
    SizeUnread = 9,
    SizeBuffer = 10,
    Unknown = 11,
}

#[derive(Clone, Copy, Debug, Pod)]
#[repr(C)]
pub struct Sysinfo {
    /// Seconds since boot
    pub uptime: usize,
    /// 1, 5, and 15 minute load averages
    pub loads: [usize; 3],
    /// Total usable main memory size
    pub totalram: usize,
    /// Available memory size
    pub freeram: usize,
    /// Amount of shared memory
    pub sharedram: usize,
    /// Memory used by buffers
    pub bufferram: usize,
    /// Total swap space size
    pub totalswap: usize,
    /// Swap space still available
    pub freeswap: usize,
    /// Number of current processes
    pub procs: u16,
    /// Total high memory size
    pub totalhigh: usize,
    /// Available high memory size
    pub freehigh: usize,
    /// Memory unit size in bytes
    pub mem_unit: u32,
    //char __reserved[256];
    // In the above structure, sizes of the memory and swap fields are given as multiples of mem_unit bytes.
}

#[repr(C)]
#[derive(Clone, Copy, Default, Pod)]
pub struct Rusage {
    /// user CPU time used
    pub ru_utime: TimeVal,
    /// system CPU time used
    pub ru_stime: TimeVal,
    /// (NOT IMPLEMENTED) maximum resident set size
    ru_maxrss: isize,
    /// (NOT IMPLEMENTED) integral shared memory size
    ru_ixrss: isize,
    /// (NOT IMPLEMENTED) integral unshared data size
    ru_idrss: isize,
    /// (NOT IMPLEMENTED) integral unshared stack size
    ru_isrss: isize,
    /// (NOT IMPLEMENTED) page reclaims (soft page faults)
    ru_minflt: isize,
    /// (NOT IMPLEMENTED) page faults (hard page faults)
    ru_majflt: isize,
    /// (NOT IMPLEMENTED) swaps
    ru_nswap: isize,
    /// (NOT IMPLEMENTED) block input operations
    ru_inblock: isize,
    /// (NOT IMPLEMENTED) block output operations
    ru_oublock: isize,
    /// (NOT IMPLEMENTED) IPC messages sent
    ru_msgsnd: isize,
    /// (NOT IMPLEMENTED) IPC messages received
    ru_msgrcv: isize,
    /// (NOT IMPLEMENTED) signals received
    ru_nsignals: isize,
    /// (NOT IMPLEMENTED) voluntary context switches
    ru_nvcsw: isize,
    /// (NOT IMPLEMENTED) involuntary context switches
    ru_nivcsw: isize,
}

#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntEnum)]
pub enum RusageFlag {
    /// Returns the resource usage of the calling process
    RusageSelf = 0,
    /// Returns the resource usage of all children of the calling process that have
    /// terminated and been waited for
    RusageChildren = -1,
    /// Returns the resource usage of the calling thread
    RusageThread = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, IntEnum)]
#[allow(non_camel_case_types)]
/// See https://github.com/torvalds/linux/blob/master/include/uapi/linux/prctl.h#L76
pub enum PrctlOp {
    /// Second arg is a signal
    PR_SET_PDEATHSIG = 1,
    /// Second arg is a ptr to return the signal
    PR_GET_PDEATHSIG = 2,
    /// Get/set current->mm->dumpable
    PR_GET_DUMPABLE = 3,
    PR_SET_DUMPABLE = 4,
    /// Get/set unaligned access control bits (if meaningful)
    ///
    /// - PR_UNALIGN_NOPRINT = 1:	 silently fix up unaligned user accesses
    /// - PR_UNALIGN_SIGBUS = 2:	 generate SIGBUS on unaligned user access
    PR_GET_UNALIGN = 5,
    PR_SET_UNALIGN = 6,
    /// Get/set whether or not to drop capabilities on setuid() away from
    ///  uid 0 (as per security/commoncap.c)
    PR_GET_KEEPCAPS = 7,
    PR_SET_KEEPCAPS = 8,
    ///  Get/set floating-point emulation control bits (if meaningful)
    ///
    /// -  PR_FPEMU_NOPRINT	=1	: silently emulate fp operations accesses
    /// -  PR_FPEMU_SIGFPE =2	: don't emulate fp operations, send SIGFPE instead
    PR_GET_FPEMU = 9,
    PR_SET_FPEMU = 10,
    /// Get/set floating-point exception mode (if meaningful)
    ///
    /// - PR_FP_EXC_SW_ENABLE = 0x80 : Use FPEXC for FP exception enables
    /// - PR_FP_EXC_DIV = 	0x010000 : floating point divide by zero
    /// - PR_FP_EXC_OVF	= 0x020000	: floating point overflow
    /// - PR_FP_EXC_UND	= 0x040000	: floating point underflow
    /// - PR_FP_EXC_RES	= 0x080000	: floating point inexact result
    /// - PR_FP_EXC_INV	= 0x100000	: floating point invalid operation
    /// - PR_FP_EXC_DISABLED	= 0	:FP exceptions disabled
    /// - PR_FP_EXC_NONRECOV	= 1	: async non-recoverable exc. mode
    /// - PR_FP_EXC_ASYNC	= 2	: async recoverable exception mode
    /// - PR_FP_EXC_PRECISE	= 3	: precise exception mode
    PR_GET_FPEXC = 11,
    PR_SET_FPEXC = 12,

    /// Get/set whether we use statistical process timing or accurate timestamp
    /// based process timing
    ///
    /// -  PR_TIMING_STATISTICAL = 0: Normal, traditional,statistical process timing
    /// - PR_TIMING_TIMESTAMP = 1:Accurate timestamp based  process timing
    PR_GET_TIMING = 13,
    PR_SET_TIMING = 14,

    /// Set/get the name of the calling thread
    PR_SET_NAME = 15,
    PR_GET_NAME = 16,
    /// Get/set process endian
    ///
    /// - PR_ENDIAN_BIG = 0: Big endian
    /// - PR_ENDIAN_LITTLE = 1: Little endian
    /// - PR_ENDIAN_PPC_LITTLE = 2: PPC Linux endian
    PR_GET_ENDIAN = 19,
    PR_SET_ENDIAN = 20,
    /// Get/set process seccomp mode
    PR_GET_SECCOMP = 21,
    PR_SET_SECCOMP = 22,
    /// Get/set the capability bounding set
    PR_CAPBSET_READ = 23,
    PR_CAPBSET_DROP = 24,
    /// Get/set the process' ability to use the timestamp counter instruction
    ///
    /// - PR_TSC_ENABLE = 1: Allow the use of the timestamp counter
    /// - PR_TSC_SIGSEGV = 2: throw a SIGSEGV instead of reading the TSC
    PR_GET_TSC = 25,
    PR_SET_TSC = 26,
    /// Get/set securebits
    PR_GET_SECUREBITS = 27,
    PR_SET_SECUREBITS = 28,
    /// Get/set the timerslack as used by poll/select/nanosleep.
    /// A value of 0 means "use default
    PR_SET_TIMERSLACK = 29,
    PR_GET_TIMERSLACK = 30,
    PR_TASK_PERF_EVENTS_DISABLE = 31,
    PR_TASK_PERF_EVENTS_ENABLE = 32,
    /// Set early/late kill mode for hwpoison memory corruption.
    /// This influences when the process gets killed on a memory corruption.
    ///
    /// - PR_MCE_KILL_CLEAR = 0
    /// - PR_MCE_KILL_SET = 1
    /// - PR_MCE_KILL_LATE = 0
    /// - PR_MCE_KILL_EARLY = 1
    /// - PR_MCE_KILL_DEFAULT = 2
    PR_MCE_KILL = 33,
    PR_MCE_KILL_GET = 34,
    /// Tune up process memory map specifics.
    ///
    /// - PR_SET_MM_START_CODE = 1
    /// - PR_SET_MM_END_CODE = 2
    /// - PR_SET_MM_START_DATA = 3
    /// - PR_SET_MM_END_DATA = 4
    /// - PR_SET_MM_START_STACK = 5
    /// - PR_SET_MM_START_BRK = 6
    /// - PR_SET_MM_BRK = 7
    /// - PR_SET_MM_ARG_START = 8
    /// - PR_SET_MM_ARG_END = 9
    /// - PR_SET_MM_ENV_START = 10
    /// - PR_SET_MM_ENV_END = 11
    /// - PR_SET_MM_AUXV = 12
    /// - PR_SET_MM_EXE_FILE = 13
    /// - PR_SET_MM_MAP = 14
    /// - PR_SET_MM_MAP_SIZE = 15
    PR_SET_MM = 35,
    PR_SET_CHILD_SUBREAPER = 36,
    PR_GET_CHILD_SUBREAPER = 37,
    PR_SET_NO_NEW_PRIVS = 38,
    PR_GET_NO_NEW_PRIVS = 39,
    PR_GET_TID_ADDRESS = 40,
    PR_SET_THP_DISABLE = 41,
    PR_GET_THP_DISABLE = 42,
    /// -  PR_FP_MODE_FR = 1 : 64b FP registers
    /// - PR_FP_MODE_FRE = 2 : 32b FP registers
    PR_SET_FP_MODE = 45,
    PR_GET_FP_MODE = 46,

    /// Control the ambient capability set
    ///
    /// - PR_CAP_AMBIENT_IS_SET = 1
    /// - PR_CAP_AMBIENT_RAISE = 2
    /// - PR_CAP_AMBIENT_LOWER = 3
    /// - PR_CAP_AMBIENT_CLEAR_ALL = 4
    PR_CAP_AMBIENT = 47,
    /// set task vector length
    PR_SVE_SET_VL = 50,
    PR_SVE_GET_VL = 51,
    /// Per task speculation control
    PR_GET_SPECULATION_CTRL = 52,
    PR_SET_SPECULATION_CTRL = 53,
    PR_PAC_RESET_KEYS = 54,
    PR_SET_TAGGED_ADDR_CTRL = 55,
    PR_GET_TAGGED_ADDR_CTRL = 56,
    /// Control reclaim behavior when allocating memory
    PR_SET_IO_FLUSHER = 57,
    PR_GET_IO_FLUSHER = 58,
    /// Dispatch syscalls to a userspace handler
    PR_SET_SYSCALL_USER_DISPATCH = 59,
    /// Set/get enabled arm64 pointer authentication keys
    ///
    PR_PAC_SET_ENABLED_KEYS = 60,
    PR_PAC_GET_ENABLED_KEYS = 61,
    /// Request the scheduler to share a core
    PR_SCHED_CORE = 62,
    PR_SME_SET_VL = 63,
    PR_SME_GET_VL = 64,
    /// Memory deny write / execute
    PR_SET_MDWE = 65,
    PR_GET_MDWE = 66,
    PR_SET_MEMORY_MERGE = 67,
    PR_GET_MEMORY_MERGE = 68,
    PR_RISCV_V_SET_CONTROL = 69,
    PR_RISCV_V_GET_CONTROL = 70,
    PR_RISCV_SET_ICACHE_FLUSH_CTX = 71,
}
