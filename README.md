# Atlio
A bash script to install linux on termux  
在Termux安装Linux的bash脚本

## Download/下载
```
pkg in wget 
wget https://raw.githubusercontent.com/YadominJinta/atilo/master/atilo -O ~/atilo
chmod +x ~/atilo
```

## Usage/使用方法
```
./atilo [option/选项] [parameter/参数]

arch           Install/安装 archlinux
fedora         Install/安装 fedora
alpine         Install/安装alpine
aosc           Install/安装aosc
ubuntu         Install/安装ubuntu
debian         Install/安装debian
kali           Install/安装kali
-r             Deletee installed Linux/删除已安装的Linux

# e.g
./atilo arch
# 等待安装/Wait
startarch
```
~~CentOS~~ 已经移除/already removed
## Note
### 1.
**Already fixed**  
Due to the problem of `termux-exec`(It's very useful but not stable now).You may meet the problem below.  
因为termux-exec(十分有用但是尚不稳定)的原因，你可能会遇到以下报错  

```
proot error: execve("/usr/bin/env"): No such file or directory
…
…
fatal error: see `proot --help`.
proot error: can't chmod '/data/data/com.termux/files/usr/tmp//proot-31860-8sDNj7': No such file or directory

```
`unset LD_PRELOAD` will fix it/能够修复它

### 2.

**All the download links is in China.**
