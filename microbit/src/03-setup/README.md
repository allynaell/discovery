<!-- # Setting up a development environment -->

# Configurando o ambiente de desenvolvimento

<!-- Dealing with microcontrollers involves several tools as we'll be dealing with an
architecture different from your computer's and we'll have to run and debug
programs on a "remote" device. -->

Lidar com microcontroladores envolve o uso de várias ferramentas, pois estaremos
lidando com uma arquitetura diferente da arquitetura do seu computador e teremos
que executar e depurar programas em um dispositivo "remoto".

<!-- ## Documentation -->

## Documentação

<!-- Tooling is not everything though. Without documentation, it is pretty much
impossible to work with microcontrollers. -->

No entanto, as ferramentas não são tudo. Sem documentação, é praticamente
impossível trabalhar com microcontroladores.

<!-- We'll be referring to all these documents throughout this book: -->

Vamos fazer referência a todos esses documentos ao longo deste livro:

- [LSM303AGR]

[LSM303AGR]: https://www.st.com/resource/en/datasheet/lsm303agr.pdf

<!-- ## Tools -->

## Ferramentas

<!-- We'll use all the tools listed below. Where a minimum version is not specified,
any recent version should work but we have listed the version we have tested. -->

Vamos utilizar todas as ferramentas listadas abaixo. Onde uma versão mínima não
for especificada, qualquer versão recente deve funcionar, mas listamos a versão
que testamos.

<!-- - Rust 1.57.0 or a newer toolchain. -->

- Rust 1.57.0 ou um toolchain mais recente.

<!-- - `gdb-multiarch`. Tested version: 10.2. Other versions will most likely work as
  well though If your distribution/platform does not have `gdb-multiarch`
  available `arm-none-eabi-gdb` will do the trick as well. Furthermore, some
  normal `gdb` binaries are built with multiarch capabilities as well, you can
  find further information about this in the sub chapters. -->

- `gdb-multiarch`. Versão testada: 10.2. Outras versões provavelmente também
  funcionarão bem, mas se a sua distribuição/plataforma não tiver o
  `gdb-multiarch` disponível, `arm-none-eabi-gdb` também servirá. Além disso,
  alguns binários normais do `gdb` são construídos com recursos
  multiarquitetura, você pode encontrar mais informações sobre isso nos
  subcapítulos.

<!-- - [`cargo-binutils`]. Version 0.3.3 or newer. -->

- [`cargo-binutils`]. Versão 0.3.3 ou mais recente.

[`cargo-binutils`]: https://github.com/rust-embedded/cargo-binutils

<!-- - [`cargo-embed`]. Version 0.18.0 or newer. -->

- [`cargo-embed`]. Version 0.18.0 ou mais recente.

[`cargo-embed`]: https://github.com/probe-rs/cargo-embed

<!-- - `minicom` on Linux and macOS. Tested version: 2.7.1. Other versions will most
  likely work as well though -->

- `minicom` para Linux e macOS. Versão testada: 2.7.1. Outras versões
  provavelmente também funcionarão.

<!-- - `PuTTY` on Windows. -->

- `PuTTY` para Windows.

<!-- Next, follow OS-agnostic installation instructions for a few of the tools: -->

Em seguida, siga as instruções de instalação independentes do sistema
operacional para algumas das ferramentas.

### `rustc` & Cargo

<!-- Install rustup by following the instructions at
[https://rustup.rs](https://rustup.rs). -->

Instale o rustup seguindo as instruções em
[https://rustup.rs](https://rustup.rs).

<!-- If you already have rustup installed double check that you are on the stable
channel and your stable toolchain is up-to-date. `rustc -V` should return a date
newer than the one shown below: -->

Se você já tiver o rustup instalado, verifique se você está no canal estável e
se sua toolchain estável está atualizada. `rustc -V` deve retornar uma data mais
recente do que a mostrada abaixo:

```console
$ rustc -V
rustc 1.53.0 (53cb7b09b 2021-06-17)
```

### `cargo-binutils`

```console
$ rustup component add llvm-tools-preview

$ cargo install cargo-binutils --vers 0.3.3

$ cargo size --version
cargo-size 0.3.3
```

### `cargo-embed`

<!-- In order to install cargo-embed, first install its
[prerequisites](https://github.com/probe-rs/probe-rs/blob/master/cargo-embed/README.md#prerequisites).
Then install it with cargo: -->

Para instalar o cargo-embed, primeiro instale seus
[pré-requisitos](https://github.com/probe-rs/probe-rs/blob/master/cargo-embed/README.md#prerequisites).
Em seguida, instale-o com cargo:

```console
$ cargo install cargo-embed --vers 0.11.0

$ cargo embed --version
cargo-embed 0.11.0
git commit: crates.io
```

<!-- ### This repository -->

### Este repositório

<!-- Since this book also contains some small Rust code bases used in various
chapters you will also have to download its source code. You can do this in one
of the following ways: -->

Como este livro também contém algumas pequenas bases de código Rust usadas em
vários capítulos, você também terá que baixar seu código-fonte. Você pode fazer
isso de uma das seguintes maneiras:

<!-- - Visit the [repository](https://github.com/rust-embedded/discovery/), click the
  green "Code" button and then the "Download Zip" one -->

- Visite o [repositório](https://github.com/rust-embedded/discovery/), clique no
  botão verde "Code" e depois no botão "Download Zip"

<!-- - Clone it using git (if you know git you presumably already have it installed)
  from the same repository as linked in the zip approach -->

- Clone-o usando git (se você conhece o git, provavelmente já o tem instalado)
  do mesmo repositório mencionado na abordagem do zip

<!-- ### OS specific instructions -->

### Instruções específicas do sistema operacional

<!-- Now follow the instructions specific to the OS you are using: -->

Agora siga as instruções específicas para o sistema operacional que você está
usando:

- [Linux](linux.md)
- [Windows](windows.md)
- [macOS](macos.md)
