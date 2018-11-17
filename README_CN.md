# Atilo
在Termux安装Linux的bash脚本

## 安装
**请前往Release页面查看最新的release**
``` bash
VERSION=1.3
pkg in curl
curl -LO https://github.com/YadominJinta/atilo/releases/download/$VERSION/atilo-cn.deb
dpkg -i atilo-cn.deb
apt install -f
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
``` bash
./atilo list
以下为可用的Linux发行版
alpine
arch
debian
fedora
kali
ubuntu
# Ubuntu 18.10
ubuntults
# Ubuntu 18.04 LTS

./atilo list --installed
已安装

alpine
```
**备注**:  
1.CentOS仅支持EN版本，CN无(因为没有镜像)  
2.Fedora 29没有提供arm架构的镜像，请运行`dnf update --releasever=29`来升级。  
## 图形
[在termux上开启图形化](https://yadominjinta.github.io/2018/07/30/GUI-on-termux.html)


## 群组
QQ:[Termux社](https://jq.qq.com/?_wv=1027&k=5jGvbsU)  
Telegram:[Termux Group ZH_CN](https://t.me/joinchat/EBPa7EI3VrfhsRu-6iJ1yw)

## Note
1.仅有有国内镜像的发行版才会加到CN版中  
2.CN版的Kali是用Debian升级得到的


