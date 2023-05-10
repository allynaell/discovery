# Linux

<!-- Here are the installation commands for a few Linux distributions. -->

Aqui estão os comandos de instalação para algumas distribuições Linux.

<!-- ## Ubuntu 20.04 or newer / Debian 10 or newer -->

## Ubuntu 20.04 ou mais recente / Debian 10 ou mais recente

<!-- > **NOTE** `gdb-multiarch` is the GDB command you'll use to debug your ARM
> Cortex-M programs -->

> **NOTA** `gdb-multiarch` é o comando GDB que você usará para depurar seus
> programas ARM Cortex-M.

```console
$ sudo apt-get install \
  gdb-multiarch \
  minicom
```

<!-- ## Fedora 32 or newer -->

## Fedora 32 ou mais recente

<!-- > **NOTE** `gdb` is the GDB command you'll use to debug your ARM Cortex-M
> programs -->

> **NOTA** `gdb` é o comando GDB que você usará para depurar seus programas ARM
> Cortex-M.

```console
$ sudo dnf install \
  gdb \
  minicom
```

## Arch Linux

<!-- > **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs -->

> **NOTA** `arm-none-eabi-gdb` é o comando GDB que você usará para depurar seus
> programas ARM Cortex-M.

```console
$ sudo pacman -S \
  arm-none-eabi-gdb \
  minicom
```

<!-- ## Other distros -->

## Outras distribuições

<!-- > **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs -->

> **NOTA** `arm-none-eabi-gdb` é o comando GDB que você usará para depurar seus
> programas ARM Cortex-M.

<!-- For distros that don't have packages for
[ARM's pre-built toolchain](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads),
download the "Linux 64-bit" file and put its `bin` directory on your path.
Here's one way to do it: -->

Para distribuições que não possuem pacotes para o
[toolchain pré-compilado do ARM](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads),
faça o download do arquivo "Linux 64 bits" e coloque o diretório `bin` no seu
caminho. Aqui está uma maneira de fazer isso:

```console
$ mkdir -p ~/local && cd ~/local
$ tar xjf /path/to/downloaded/file/gcc-arm-none-eabi-9-2020-q2-update-x86_64-linux.tar.bz2
```

<!-- Then, use your editor of choice to append to your `PATH` in the appropriate
shell init file (e.g. `~/.zshrc` or `~/.bashrc`): -->

Em seguida, use o editor de sua escolha para adicionar ao seu `PATH` no arquivo
de inicialização do shell apropriado (por exemplo, `~/.zshrc` ou `~/.bashrc`):

```
PATH=$PATH:$HOME/local/gcc-arm-none-eabi-9-2020-q2-update/bin
```

<!-- ## udev rules -->

## Regras udev

<!-- These rules let you use USB devices like the micro:bit without root privilege,
i.e. `sudo`. -->

Essas regras permitem que você use dispositivos USB como o micro:bit sem
privilégios de root, ou seja, sem usar o comando `sudo`.

<!-- Create this file in `/etc/udev/rules.d` with the content shown below. -->

Crie este arquivo em `/etc/udev/rules.d` com o conteúdo mostrado abaixo.

```console
$ cat /etc/udev/rules.d/99-microbit.rules
```

```text
# CMSIS-DAP for microbit
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
```

<!-- Then reload the udev rules with: -->

Então, recarregue as regras do udev com o seguinte comando:

```console
$ sudo udevadm control --reload-rules
```

<!-- If you had any board plugged to your computer, unplug them and then plug them in
again. -->

Se você tiver alguma placa conectada ao seu computador, desconecte-as e, em
seguida, conecte-as novamente.

<!-- Now, go to the [next section]. -->

Agora, vá para a [próxima seção].

<!-- [next section]: verify.md -->

[próxima seção]: verify.md
