# Atilo
在Termux安装Linux的bash脚本

## 安装
``` bash
echo "deb [trusted=yes] https://yadominjinta.github.io/files/ termux    extras" >> $PREFIX/etc/apt/sources.list
pkg in atilo-cn
```

## 使用方法
``` bash
atilo [命令] [参数]
Atilo 是一个用来帮助你在termux上安装不同的GNU/Linux发行版的bash脚本。

命令:
list             列出可用的和已安装的发行版
remove           移除已安装的发行版
install          安装发行版
help             帮助
作者 @YadominJinta @seashell11234455
本atilo具有超级牛力
```

## 支持的发行版

| 发行版 | aarch64 | arm | x86_64 | i686 |
| ------------ | :-----: |:-: | :-----: | :--: |
|Arch          |√        |√   |×        |×     |
|Alpine|√|√|√|√|
|CentOS|√|√|√|√|
|Debian|√|√|√|√|
|Fedora|√|√|√|×|
|Kali|√|√|√|√|
|OpenSuSE|√|×|√|√
|Ubuntu Cosmic|√|√|√|√|
|Ubuntu LTS|√|√|√|√|



**备注**:    
1.Fedora 29没有提供arm架构的镜像，请运行`dnf update --releasever=29`来升级。  
## 图形
[在termux上开启图形化](https://yadominjinta.github.io/2018/07/30/GUI-on-termux.html)


## 群组
QQ:[Termux社](https://jq.qq.com/?_wv=1027&k=5jGvbsU)  
Telegram:[Termux Group ZH_CN](https://t.me/joinchat/EBPa7EI3VrfhsRu-6iJ1yw)

## Note
1.仅有有国内镜像的发行版才会加到CN版中  
2.CN版的Kali是用Debian升级得到的

## 相关项目
**[EXALAB/AnLinux-App](https://github.com/EXALAB/AnLinux-App)**: APP to help install Linux on termux.  
**[sdrausty/TermuxArch](https://github.com/sdrausty/TermuxArch)**: Arch install script  
**[Neo-Oli/termux-ubuntu](https://github.com/Neo-Oli/termux-ubuntu)**: Ubuntu chroot on termux  
**[Hax4us/Nethunter-In-Termux](https://github.com/Hax4us/Nethunter-In-Termux)**: Install Kali nethunter (Kali Linux) in your termux application without rooted phone  
**[nmilosev/termux-fedora](https://github.com/nmilosev/termux-fedora)**: A script to install a Fedora chroot into Termux  
**[sp4rkie/debian-on-termux](https://github.com/sp4rkie/debian-on-termux)**: Install Debian 9 (stretch) on your Android smartphone
**[Hax4us/TermuxAlpine](https://github.com/Hax4us/TermuxAlpine)**: Use TermuxAlpine.sh calling to install Alpine Linux in Termux on Android

**[Proot简明手册](https://github.com/myfreess/Mytermuxdoc/wiki/Proot)**:帮助Termux用户编写proot脚本的简明指南
