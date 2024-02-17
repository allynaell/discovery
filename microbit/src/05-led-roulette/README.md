<!-- # LED roulette -->

# Roleta LED

<!-- Alright, let's start by building the following application: -->

Certo, vamos começar construindo a seguinte aplicação:

<p align="center">
<video src="../assets/roulette_fast.mp4" loop autoplay>
</p>

<!-- I'm going to give you a high level API to implement this app but don't worry
we'll do low level stuff later on. The main goal of this chapter is to get
familiar with the _flashing_ and debugging process. -->

Vou fornecer a você uma API de alto nível para implementar este aplicativo, mas
não se preocupe, faremos coisas de baixo nível posteriormente. O objetivo
principal deste capítulo é familiarizar-se com o processo de _flashing_ e
depuração.

<!-- The starter code is in the `src` directory of the book repository. Inside that
directory there are more directories named after each chapter of this book. Most
of those directories are starter Cargo projects. -->

O código inicial está no diretório `src` do repositório do livro. Dentro desse
diretório, existem mais diretórios com nomes de cada capítulo deste livro. A
maioria desses diretórios são projetos iniciais de Cargo.

<!-- Now, jump into the `src/05-led-roulette` directory. Check the `src/main.rs`
file: -->

Agora, vá para o diretório `src/05-led-roulette`. Verifique o arquivo
`src/main.rs`:

```rust
{{#include src/main.rs}}
```

<!-- Microcontroller programs are different from standard programs in two aspects:
`#![no_std]` and `#![no_main]`. -->

Programas para microcontroladores são diferentes de programas convencionais em
dois aspectos: `#![no_std]` e `#![no_main]`.

<!-- The `no_std` attribute says that this program won't use the `std` crate, which
assumes an underlying OS; the program will instead use the `core` crate, a
subset of `std` that can run on bare metal systems (i.e., systems without OS
abstractions like files and sockets). -->

O atributo `no_std` indica que este programa não usará o crate `std`, que
pressupõe a existência de um sistema operacional subjacente. Em vez disso, o
programa usará o crate `core`, que é um subconjunto do `std` que pode ser
executado em sistemas bare metal (ou seja, sistemas sem abstrações de sistema
operacional, como arquivos e sockets).

<!-- The `no_main` attribute says that this program won't use the standard `main`
interface, which is tailored for command line applications that receive
arguments. Instead of the standard `main` we'll use the `entry` attribute from
the [`cortex-m-rt`] crate to define a custom entry point. In this program we
have named the entry point "main", but any other name could have been used. The
entry point function must have signature `fn() -> !`; this type indicates that
the function can't return -- this means that the program never terminates. -->

O atributo `no_main` indica que este programa não usará a interface `main`
padrão, que é adequada para aplicativos de linha de comando que recebem
argumentos. Em vez do `main` padrão, utilizaremos o atributo `entry` do crate
[cortex-m-rt] para definir um ponto de entrada customizado. Neste programa,
nomeamos o ponto de entrada como "main", mas poderíamos ter usado qualquer outro
nome. A função de ponto de entrada deve ter a assinatura `fn() -> !`, indicando
que a função não pode retornar -- isso significa que o programa nunca termina.

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

<!-- If you are a careful observer, you'll also notice there is a `.cargo` directory
in the Cargo project as well. This directory contains a Cargo configuration file
(`.cargo/config`) that tweaks the linking process to tailor the memory layout of
the program to the requirements of the target device. This modified linking
process is a requirement of the `cortex-m-rt` crate. -->

Se você for um observador cuidadoso, também notará que existe um diretório
`.cargo` no projeto Cargo. Esse diretório contém um arquivo de configuração do
Cargo (`.cargo/config`) que ajusta o processo de linking para adaptar o layout
de memória do programa aos requisitos do dispositivo alvo. Esse processo de
linking modificado é um requisito do crate `cortex-m-rt`.

<!-- Furthermore, there is also an `Embed.toml` file -->

Além disso, há também um arquivo `Embed.toml`.

```toml
{{#include Embed.toml}}
```

<!-- This file tells `cargo-embed` that: -->

Este arquivo informa ao `cargo-embed` que:

<!-- - we are working with either a nrf52833 or nrf51822, you will again have to
  remove the comments from the chip you are using, just like you did in
  chapter 3. -->

- Estamos trabalhando com um nrf52833 ou nrf51822. Você terá que remover
  novamente os comentários do chip que estiver usando, assim como fez no
  capítulo 3.

<!-- - we want to halt the chip after we flashed it so our program does not instantly
  jump to the loop -->

- Desejamos interromper o chip depois de gravá-lo para que nosso programa não
  salte instantaneamente para o loop.

<!-- - we want to disable RTT, RTT being a protocol that allows the chip to send text
  to a debugger. You have in fact already seen RTT in action, it was the
  protocol that sent "Hello World" in chapter 3. -->

- Queremos desabilitar o RTT, que é um protocolo que permite ao chip enviar
  texto para um depurador. Na verdade, você já viu o RTT em ação: ele foi o
  protocolo que enviou "Hello World" no capítulo 3.

<!-- - we want to enable GDB, this will be required for the debugging procedure -->

- Desejamos habilitar o GDB, pois isso será necessário para o procedimento de
  depuração.

<!-- Alright, let's start by building this program. -->

Certo, vamos começar construindo este programa.
