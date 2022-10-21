---
title: "类原生编译入门"
description: 手把手教你编译安卓系统
date: 2022-10-14
image: "image.png"
slug: build-android
categories:
    - aosp
---

## 前言

这半年的时间都在玩安卓类原生，总的来说构建安卓系统并不是很难，这里就记录下怎么编译一个安卓类原生，这里以给 联想小新 Pad Plus 编译 Pixel Experience 作为例子

## 准备

### 配置要求

编译安卓系统对于配置的要求是相当高的，尤其是对于内存和硬盘有硬性要求，这里说下我用的配置

```
CPU: Intel Core(R) i7-8750H
内存: 16G (8G x2) DDR4 2333Mhz
硬盘: 1TB M.2 PCIE3.0 x2
```

简单说下要求吧

#### 内存

谷歌官方推荐最少 16G，如果你和我一样是 16G 的内存，那么你还需要分配一定大小的Swap，这里建议分配 16G，实际编译时大致的内存和 Swap 消耗大致为 15G + 9G，如果 Swap 或者 内存 太少，那么在 soong 的 build 过程可能就会失败

#### 硬盘

这里我建议你使用固态硬盘，留有至少 300GB 的剩余空间，如果你想使用机械硬盘，避免使用叠瓦盘，编译过程对硬盘的写入量可以说跟挖矿差不多了，硬盘的IO效率对编译的速度有着直接的影响，考虑到编译过程的巨大IO量，叠瓦盘可能很难经受得住（我自己就坏了一块），所以尽量避免叠瓦盘

#### CPU

CPU 没啥好说的，越好的 CPU 编译就越快，建议最少 8 个线程的 CPU，给一个耗时参考吧（虽然可能并没有什么用）

| CPU | Thread | Time |
|-----|--------|------|
| Intel Core(R) i7-8750H | 12 | 3\.5h\~4h |
| AMD EPYC(R) Rome Processor | 8 | 2\.8h\~3h |

### 环境配置

操作系统的话我使用的是 Arch Linux，之所以没有选择 Ubuntu 是因为 Ubuntu 在安装依赖的时候会掺不少垃圾进来，造成不少内存和硬盘资源的浪费（其实主要是我用 Arch Linux 用习惯了），因此本教程以 Arch Linux 作为示范

#### 安装工具链

首先我们需要安装 Arch Linux 的基本开发工具组

```shell
$ sudo pacman -Syu base-devel
```

接下来我们配置一下 Arch Linux CN 源，有部分库我们需要从 AUR 获取，而我们接下来要用到的 AUR 包管理器 paru 可以从 Arch Linux CN 获取，当然你也可以选择自行编译，不过能简单点为什么不简单点呢？

我们编辑下 `/etc/pacman.conf`

```shell
$ sudo nano /etc/pacman.conf
```

在最后加上如下内容

```
[archlinuxcn]
Server = https://mirrors.bfsu.edu.cn/archlinuxcn/$arch
```

Ctrl + O 保存，Ctrl + X 退出，然后刷新软件包缓存，安装密钥链

```shell
$ sudo pacman -Syy
$ sudo pacman -S archlinuxcn-keyring
```

安装 paru

```shell
$ sudo pacman -S paru
```

接下来我们通过 paru 安装 lineageos-devel，要特别注意的是，不要以 root 权限运行 paru，需要使用 root 权限的时候程序会要你输入密码的

```shell
$ paru -S lineageos-devel
```

一路回车下去，弹出窗口显示 PKGBUILD 信息的时候输入 `q` 然后按回车关闭 vi 编辑器，继续一路回车下去，完成安装

#### 安装 repo

这个东西是用来获取安卓源代码的，这是谷歌使用的源代码仓库管理工具，因为安卓是由很多个项目组成的，这个工具将帮助我们一键下载所有需要的项目

repo 可以从谷歌代码仓库获取

```shell
$ mkdir -p ~/.local/bin
$ curl https://storage.googleapis.com/git-repo-downloads/repo > ~/.local/bin/repo
$ chmod a+x ~/.local/bin/repo
```

接下来我们修改下环境变量，将 repo 的安装路径添加进 PATH，这里要修改的文件取决于你用了什么 shell，如果你用的是 bash（一般默认是 bash），那么请修改 `~/.bashrc`，如果你用的是 zsh，那么请修改 `~/.zshrc`，这里以 zsh 为例

```shell
$ echo 'export PATH=$PATH:$HOME/.local/bin' >> ~/.zshrc
$ source ~/.zshrc
```

## 正式开工
### 获取源码
#### 创建工作目录

首先我们先来创建一个工作目录

```shell
$ mkdir -p ~/build_area
```

切换到工作目录

```shell
$ cd ~/build_area
```

#### 配置加速镜像

众所周知的，在国内由于不可抗力因素，我们无法直接获取到源代码，这个时候我们可以选择使用魔法，或者使用加速镜像，这里我建议是使用魔法的，因为加速镜像随时可能会抽风，但这里我依然以加速镜像作为例子，因为不同魔法配置起来可能会有所不同

```shell
$ git config --global url.https://mirrors.bfsu.edu.cn/git/AOSP/.insteadof https://android.googlesource.com
$ git config --global url.https://hub.fastgit.xyz/.insteadof https://github.com
$ echo "export REPO_URL='https://mirrors.bfsu.edu.cn/git/git-repo'" >> ~/.zshrc
$ source ~/.zshrc
```

#### 获取源码

##### 获取系统源码

我们初始化一下本地仓库，因为我们这里只是用来编译，而不是用来开发，我们不需要获取完整的提交记录，因此在这里加上 `--depth=1` 参数仅获取最新的提交

```shell
$ repo init -u https://github.com/PixelExperience/manifest -b thirteen --depth=1
```

接下来我们获取源码

```shell
$ repo sync -c -j$(nproc --all) --force-sync --no-clone-bundle --no-tags
```

##### 获取必要的构建材料

构建安卓系统需要三个必须的材料：Device Configuration、Vendor Blobs、Kernel Source

其中 Device Configuration 我们习惯上称之为 Device Tree，也就是设备树，里面包含有设备的一些配置信息，当然里面也可能有加入一些必要的库的源码；Vendor Blobs 则是构建安卓系统所需要的一些闭源的二进制文件，对于大部分的设备而言，我们并没有能够构建完整安卓系统的源码，这个时候缺失的部分我们就需要从官方的系统中提取了，这个就是 Vendor Blobs；Kernel Source 就不需要多讲了，这个就是内核

我们获取这些材料，并放到相应的目录，如设备树就放到 `device/厂商/机型代号`，如果你的设备没有这些东西，那么就只能自己手搓了，这篇文章暂时不讲这个，我可能以后有空会讲讲

```shell
$ git clone https://github.com/bigsaltyfishes/device_lenovo_J607F -b pe-13.0 device/lenovo/J607F --depth=1
$ git clone https://github.com/bigsaltyfishes/kernel_lenovo_J607Z -b kernel-next kernel/lenovo/J607Z --depth=1
$ git clone https://github.com/bigsaltyfishes/vendor_lenovo_J607Z -b thirteen vendor/lenovo/J607Z --depth=1
```

有些设备使用了通用设备树的则需要将通用设备树一起拉取下来，这里就不做示范了，方法一样

因为联想在内核里面掺了些奇奇怪怪的东西，我也不好随便修改，因此里面引用的一个没用的构建内核模块用的 Makefile 我也没有清理掉（其实就是我懒😏），所以我们需要手动新建一个空的 Makefile 避免报错

```shell
$ mkdir -p device/qcom/common/dlkm
$ touch device/qcom/common/dlkm/AndroidKernelModule.mk
```

我的内核使用了 Proton Clang 进行编译，我们需要拉取这个工具链下来，不同开发者使用的工具链可能不同，根据情况来获取相应的工具链即可（也有可能会用 AOSP 自带的，自带的就跳过这步吧）

```shell
$ git clone https://github.com/kdrag0n/proton-clang.git prebuilts/clang/host/linux-x86/proton-clang --depth=1
```


### 构建系统
我们先配置下 CCACHE，这个东西其实就是个缓存，虽然可以不用它，但是用了编译会快不少，所以还是用吧，我们把缓存大小限制在 50GB

```shell
$ ccache -M 50G
```

在环境变量中定义使用 CCACHE，并启用缓存压缩

```shell
$ export USE_CCACHE=1
$ export CCACHE_COMPRESS=1
```

接下来我们初始化构建环境

```shell
$ . build/envsetup.sh
```

选择构建目标，这里我们以 userdebug 目标为例，如果想知道有什么可用的构建目标的话，可以在设备树目录下的 AndroidProducts.mk 中查看

```shell
$ lunch aosp_J607F-userdebug
```

编译系统

```shell
$ mka bacon
```

不出意外的话，几个小时后系统就能编译完成了，产物会保存在 `out/target/product/机型代号` 目录下，然后正常卡刷就可以了