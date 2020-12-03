# Atilo

A program to install linux on termux  

![Test](https://github.com/YadominJinta/atilo/workflows/Test/badge.svg)
  
中文用户[点击这里](https://github.com/YadominJinta/atilo/blob/master/CN/README_CN.md)

## Installation

``` bash
echo "deb [trusted=yes arch=all] https://yadominjinta.github.io/files/ termux extras" >> $PREFIX/etc/apt/sources.list.d/atilo.list
apt update && apt install atilo
```

## Usage

``` bash
atilo [command] [Arguments]
Atilo           2.0
Usage: atilo [Command] [Argument]

Atilo is a program to help you install some GNU/Linux distributions on Termux.

Commands:
images           list available images
remove           remove installed images
pull             pulling an image
run              run an image
clean            clean tmps
help             show this help.
```

## Support Linux

| Distribution  | aarch64 |  arm  | x86_64 | i686  |
| ------------- | :-----: | :---: | :----: | :---: |
| Alpine        |    √    |   √   |   √    |   √   |
| CentOS        |    √    |   ×   |   √    |   ×   |
| Debian        |    √    |   √   |   √    |   √   |
| Fedora        |    √    |   √   |   √    |   ×   |
| Kali          |    √    |   √   |   √    |   √   |
| openSUSE      |    √    |   √   |   ×    |   ×   |
| Ubuntu        |    √    |   √   |   √    |   √   |

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
