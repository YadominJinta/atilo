# Atilo
A bash script to install linux on termux  
  
中文用户[点击这里](https://github.com/YadominJinta/atilo/blob/master/CN/README_CN.md)
## Installation
``` bash
echo "deb [trusted=yes] https://yadominjinta.github.io/files/ termux extras" >> $PREFIX/etc/apt/sources.list
pkg in atilo
```

## Usage
``` bash
atilo [command] [Arguments]
Atilo is a bash script to help you install some GNU/Linux distributions on Termux.

Commands:
list             list available distributions
   --installed   list installed distributions
remove           remove installed distributions
install          install distributions
help             Show this help
Writen by @YadominJinta @seashell11234455

This atilo has super cow power
```

## Support Linux
| Distribution | aarch64 | arm | x86_64 | i686 |
| ------------ | :-----: |:-: | :-----: | :--: |
|Arch          |√        |√   |×        |×     |
|Alpine|√|√|√|√|
|CentOS|√|√|√|√|
|Debian|√|√|√|√|
|Fedora|√|√|√|×|
|Kali|√|√|√|√|
|OpenSuSE|√|√|×|×|
|ParrotOS|√|√|√|√|
|Ubuntu Cosmic|√|√|√|√|
|Ubuntu LTS|√|√|√|√|

**Notice**: Fedora 29 doesn't provode an image for arm,so you have to run `dnf update --releasever=29` to update to 29.

**Notice for parrot**:If you want to add Tuna mirror for your parrot,please run

```shell
cat > /etc/apt/sources.list <<<"deb http://mirrors.tuna.tsinghua.edu.cn/parrot/ parrot main contrib non-free"
```

## GUI

[Using GUI on termux](https://yadominjinta.github.io/2018/08/18/GUI-on-termux-EN.html)

## Group
Telegram:[Termux Group ZH_CN](https://t.me/joinchat/EBPa7EI3VrfhsRu-6iJ1yw)

# Relate Projects
**[EXALAB/AnLinux-App](https://github.com/EXALAB/AnLinux-App)**: APP to help install Linux on termux.  
**[sdrausty/TermuxArch](https://github.com/sdrausty/TermuxArch)**: Arch install script  
**[Neo-Oli/termux-ubuntu](https://github.com/Neo-Oli/termux-ubuntu)**: Ubuntu chroot on termux  
**[Hax4us/Nethunter-In-Termux](https://github.com/Hax4us/Nethunter-In-Termux)**: Install Kali nethunter (Kali Linux) in your termux application without rooted phone  
**[nmilosev/termux-fedora](https://github.com/nmilosev/termux-fedora)**: A script to install a Fedora chroot into Termux  
**[sp4rkie/debian-on-termux](https://github.com/sp4rkie/debian-on-termux)**: Install Debian 9 (stretch) on your Android smartphone

**[Hax4us/TermuxAlpine](https://github.com/Hax4us/TermuxAlpine)**: Use TermuxAlpine.sh calling to install Alpine Linux in Termux on Android  
