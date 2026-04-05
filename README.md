# Isolation in os subsystem



## 1. Introduction



## 2. Implementation

   
## 3. Run
- rust
- riscv64-linux-musl-gcc
- git submodule update --init --recursive
- mtools (mcopy/mmd/mdir, for sudo-free sdcard image packaging)

```
make run
```

`make run` 路径中的 `sdcard` 打包已改为通过 mtools 直接写入镜像文件，不再依赖 `sudo mount/umount`。

```
make build        # build kernel
make sdcard       # build all domains and user app
make initrd       # build initrd (choose static busybox)
```


### Run GUI app
```
make run GUI=y
```

```shell
# load gpu module
> dgpu
# load input device module
> dinput
# run memory_game/slint app
> memory_game
```

You can press `Esc` to exit the app. 

### Load domain from network
```shell
# load net module
> dnet
# run out app to trigger log output
> out
# load new log domain from domain server
> dlog new
# run out app to see new log domain output
> out
# replay old log domain
> dlog old
```


## Reference
[git submodule](https://iphysresearch.github.io/blog/post/programing/git/git_submodule/)