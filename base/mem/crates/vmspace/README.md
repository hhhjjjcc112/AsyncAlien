# vmspace


## Example

```rust
#[derive(Debug)]
pub struct VmmPageAllocator;

impl PagingIf<Rv64PTE> for VmmPageAllocator {
    fn alloc_frame() -> Option<Box<dyn NotLeafPage<Rv64PTE>>> {
    }
}

let mut space = VmSpace::<VmmPageAllocator>::new();

```
