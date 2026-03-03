use bitflags::bitflags;
use int_enum::IntEnum;
use pod::Pod;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct EpollEvent {
    pub events: EpollEventType,
    pub data: u64,
}

bitflags! {
    pub struct EventFdFlags: u32{
        const EFD_SEMAPHORE = 1;
        const EFD_CLOEXEC = 0o2000000;
        const EFD_NONBLOCK = 0o0004000;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default,Pod)]
    pub struct EpollEventType: u32 {
        const EPOLLIN = 0x001;
        const EPOLLPRI = 0x002;
        const EPOLLOUT = 0x004;
        const EPOLLRDNORM = 0x040;
        const EPOLLRDBAND = 0x080;
        const EPOLLWRNORM = 0x100;
        const EPOLLWRBAND = 0x200;
        const EPOLLMSG = 0x400;
        const EPOLLERR = 0x008;
        const EPOLLHUP = 0x010;
        const EPOLLRDHUP = 0x2000;
        const EPOLLEXCLUSIVE = 1 << 28;
        const EPOLLWAKEUP = 1 << 29;
        const EPOLLONESHOT = 1 << 30;
        const EPOLLET = 1 << 31;
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, IntEnum)]
pub enum EpollCtlOp {
    /// Add an entry to the interest list of the epoll file descriptor, epfd
    EpollCtlAdd = 1,
    /// Remove (deregister) the target file descriptor fd from the interest list.
    EpollCtlDel = 2,
    /// Change the settings associated with fd in the interest
    /// list to the new settings specified in event.
    EpollCtlMod = 3,
}
