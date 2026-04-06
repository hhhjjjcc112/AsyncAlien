mod dir;
mod file;

use alloc::sync::Weak;
use fatfs::FileAttributes;

pub use dir::*;
pub use file::*;
use vfscore::utils::VfsNodePerm;

use crate::{fs::FatFsSuperBlock, *};

const FAT_DEFAULT_DIR_MODE: u16 = 0o755;
const FAT_DEFAULT_FILE_MODE: u16 = 0o755;

fn clear_write_perm(perm: &mut VfsNodePerm) {
    perm.remove(VfsNodePerm::OWNER_WRITE | VfsNodePerm::GROUP_WRITE | VfsNodePerm::OTHER_WRITE);
}

pub(crate) fn fat_dir_perm(attrs: FileAttributes) -> VfsNodePerm {
    let mut perm = VfsNodePerm::from_bits_truncate(FAT_DEFAULT_DIR_MODE);
    if attrs.contains(FileAttributes::READ_ONLY) {
        clear_write_perm(&mut perm);
    }
    perm
}

pub(crate) fn fat_file_perm(attrs: FileAttributes) -> VfsNodePerm {
    let mut perm = VfsNodePerm::from_bits_truncate(FAT_DEFAULT_FILE_MODE);
    if attrs.contains(FileAttributes::READ_ONLY) {
        clear_write_perm(&mut perm);
    }
    perm
}

pub(crate) fn fat_root_dir_perm() -> VfsNodePerm {
    VfsNodePerm::from_bits_truncate(FAT_DEFAULT_DIR_MODE)
}

struct FatFsInodeSame<R: VfsRawMutex> {
    pub sb: Weak<FatFsSuperBlock<R>>,
    pub inner: Mutex<R, FatFsInodeAttr>,
}
struct FatFsInodeAttr {
    pub atime: VfsTimeSpec,
    pub mtime: VfsTimeSpec,
    pub ctime: VfsTimeSpec,
    pub perm: VfsNodePerm,
}

impl<R: VfsRawMutex> FatFsInodeSame<R> {
    pub fn new(sb: &Arc<FatFsSuperBlock<R>>, perm: VfsNodePerm) -> Self {
        Self {
            sb: Arc::downgrade(sb),
            inner: Mutex::new(FatFsInodeAttr {
                atime: VfsTimeSpec::new(0, 0),
                mtime: VfsTimeSpec::new(0, 0),
                ctime: VfsTimeSpec::new(0, 0),
                perm,
            }),
        }
    }
}
