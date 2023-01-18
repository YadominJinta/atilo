# Atilo

在Termux安装Linux

![Test](https://github.com/YadominJinta/atilo/workflows/Test/badge.svg)

## 安装

``` bash
echo "deb [trusted=yes arch=all] https://yadominjinta.github.io/files/ termux extras" >> $PREFIX/etc/apt/sources.list.d/atilo.list
apt update && apt install atilo-cn
```

## 使用方法

``` bash
Atilo           2.0
Usage: atilo [命令] [参数]

Atilo 是一个用来帮助你在termux上安装不同的GNU/Linux发行版的程序

命令:
images           列出可用镜像
remove           移除本地的镜像
pull             拉取远的镜像
run              运行镜像
clean            清除缓存
help             帮助
```

## 支持的发行版

| 发行版        | aarch64 |  arm  | x86_64 | i686  |
| ------------- | :-----: | :---: | :----: | :---: |
| Alpine        |    √    |   √   |   √    |   √   |
| CentOS        |    √    |   √   |   √    |   √   |
| Debian        |    √    |   √   |   √    |   √   |
| Fedora        |    √    |   √   |   √    |   ×   |
| Kali          |    √    |   √   |   √    |   √   |
| openSUSE      |    √    |   ×   |   √    |   √   |
| Ubuntu        |    √    |   √   |   √    |   √   |

## 图形

[在termux上开启图形化](https://yadominjinta.github.io/2018/07/30/GUI-on-termux.html)

## 群组

QQ:[Termux社](https://jq.qq.com/?_wv=1027&k=5jGvbsU)  
Telegram:[Termux Group ZH_CN](https://t.me/joinchat/EBPa7EI3VrfhsRu-6iJ1yw)

## 相关项目

**[EXALAB/AnLinux-App](https://github.com/EXALAB/AnLinux-App)**: APP to help install Linux on termux.  
**[sdrausty/TermuxArch](https://github.com/sdrausty/TermuxArch)**: Arch install script  
**[Neo-Oli/termux-ubuntu](https://github.com/Neo-Oli/termux-ubuntu)**: Ubuntu chroot on termux  
**[Hax4us/Nethunter-In-Termux](https://github.com/Hax4us/Nethunter-In-Termux)**: Install Kali nethunter (Kali Linux) in your termux application without rooted phone  
**[nmilosev/termux-fedora](https://github.com/nmilosev/termux-fedora)**: A script to install a Fedora chroot into Termux  
**[sp4rkie/debian-on-termux](https://github.com/sp4rkie/debian-on-termux)**: Install Debian 9 (stretch) on your Android smartphone
**[Hax4us/TermuxAlpine](https://github.com/Hax4us/TermuxAlpine)**: Use TermuxAlpine.sh calling to install Alpine Linux in Termux on Android

**[Proot简明手册](https://github.com/myfreess/Mytermuxdoc/wiki/Proot)**:帮助Termux用户编写proot脚本的简明指南
