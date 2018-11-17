# Atilo
A bash script to install linux on termux  
  
中文用户[点击这里](https://github.com/YadominJinta/atilo/blob/master/README_CN.md)
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
``` bash
./atilo list
The followings are available

alpine
arch
centos
debian
fedora
kali
ubuntu
# Ubuntu Cosmic
ubuntults
# Ubuntu Bionic LTS

./atilo list --installed
Installed 

alpine
```
**Notice**: Fedora 29 doesn't provode an image for arm,so you have to run `dnf update --releasever=29` to update to 29.
## GUI

[Using GUI on termux](https://yadominjinta.github.io/2018/08/18/GUI-on-termux-EN.html)

## Group
Telegram:[Termux Group ZH_CN](https://t.me/joinchat/EBPa7EI3VrfhsRu-6iJ1yw)



