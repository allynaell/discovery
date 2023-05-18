<!-- # Rust Embedded terminology -->

# Terminologia de Rust para Sistemas Embarcados

<!-- Before we dive into programming the micro:bit let's have a quick look at the
libraries and terminology that will be important for all the future chapters. -->

Antes de mergulharmos na programação do micro:bit, vamos dar uma rápida olhada
nas bibliotecas e terminologia que serão importantes para todos os capítulos
futuros.

<!-- ## Abstraction layers -->

## Camadas de Abstração

<!-- For any fully supported microcontroller/board with a microcontroller you will
usually hear the following terms being used for their levels of abstraction: -->

Para qualquer microcontrolador/placa totalmente suportado com um
microcontrolador, geralmente se ouvirá os seguintes termos sendo usados para
seus níveis de abstração:

<!-- ### Peripheral Access Crate (PAC) -->

### Peripheral Access Crate (PAC)

<!-- The job of the PAC is to provide a safe (ish) direct interface to the
peripherals of the chip, allowing you to configure every last bit however you
want (of course also in wrong ways). Usually you only ever have to deal with the
PAC if either the layers that are higher up don't fulfill your needs or when you
are developing them. The PAC we are (implicitly) going to use is either the one
for the [nRF52] or for the [nRF51]. -->

O trabalho do PAC é fornecer uma interface direta (mais ou menos segura) aos
periféricos do chip, permitindo configurar cada último bit da forma desejada (é
claro, também de maneiras incorretas). Normalmente, você só precisa lidar com o
PAC se as camadas acima não atenderem às suas necessidades ou quando você as
está desenvolvendo. O PAC que estamos (implicitamente) usando é o do [nRF52] ou
do [nRF51].

<!-- ### The Hardware Abstraction Layer (HAL) -->

### Camada de Abstração de Hardware (HAL)

<!-- The job of the HAL is to build up on top of the chip's PAC and provide an
abstraction that is actually usable for someone who does not know about all the
special behaviour of this chip. Usually they abstract whole peripherals away
into single structs that can for example be used to send data around via the
peripheral. We are going to use the [nRF52-hal] or the [nRF51-hal] respectively. -->

O trabalho do HAL é construir em cima do PAC do chip e fornecer uma abstração
que seja realmente utilizável para alguém que não conhece todo o comportamento
especial desse chip. Geralmente, eles abstraem periféricos inteiros em
estruturas únicas que podem, por exemplo, ser usadas para enviar dados através
do periférico. Vamos usar o [nRF52-hal] ou o [nRF51-hal], respectivamente.

<!-- ### The Board Support Crate (historically called Board Support Package, or BSP) -->

### Board Support Crate (historicamente chamado de Board Support Package, ou BSP)

<!-- The job of the BSP is to abstract a whole board (such as the micro:bit) away at
once. That means it has to provide abstractions to use both the microcontroller
as well as the sensors, LEDs etc. that might be present on the board. Quite
often (especially with custom-made boards) you will just be working with a HAL
for the chip and build the drivers for the sensors either yourself or search for
them on crates.io. Luckily for us though, the micro:bit does actually have a
[BSP] so we are going to use that on top of our HAL as well. -->

O trabalho do BSP é abstrair completamente uma placa (como o micro:bit) de uma
vez. Isso significa que ele precisa fornecer abstrações para usar tanto o
microcontrolador quanto os sensores, LEDs, etc. que podem estar presentes na
placa. Com frequência (especialmente com placas feitas sob medida), você estará
trabalhando apenas com um HAL para o chip e construirá os drivers para os
sensores por conta própria ou os procurará no crates.io. Felizmente para nós, o
micro:bit realmente tem um [BSP], então vamos usá-lo em cima do nosso HAL
também.

[nrF52]: https://crates.io/crates/nrf52833-pac
[nrF51]: https://crates.io/crates/nrf51
[nrF52-hal]: https://crates.io/crates/nrf52833-hal
[nrF51-hal]: https://crates.io/crates/nrf51-hal
[BSP]: https://crates.io/crates/microbit

<!-- ## Unifying the layers -->

## Unificando as Camadas

<!-- Next we are going to have a look at a very central piece of software in the Rust
Embedded world: [`embedded-hal`]. As its name suggests it relates to the 2nd
level of abstraction we got to know: the HALs. The idea behind [`embedded-hal`]
is to provide a set of traits that describe behaviour which is usually shared
across all implementations of a specific peripheral in all the HALs. For example
one would always expect to have functions that are capable of turning the power
on a pin either on or off. For example to switch an LED on and off on the board.
This allows us to write a driver for, say a temperature sensor, that can be used
on any chip for which an implementation of the [`embedded-hal`] traits exists,
simply by writing the driver in such a way that it only relies on the
[`embedded-hal`] traits. Drivers that are written in such a way are called
platform agnostic and luckily for us most of the drivers on crates.io are
actually platform agnostic. -->

A seguir, vamos dar uma olhada em uma peça de software muito central no mundo do
Rust para sistemas embarcados: o [`embedded-hal`]. Como o nome sugere, ele se
relaciona com o segundo nível de abstração que conhecemos: os HALs. A ideia por
trás do [`embedded-hal`] é fornecer um conjunto de traits que descrevem
comportamentos que geralmente são compartilhados em todas as implementações de
um periférico específico em todos os HALs. Por exemplo, sempre se espera ter
funções capazes de ligar ou desligar a alimentação de um pino, por exemplo, para
acender e apagar um LED na placa. Isso nos permite escrever um driver para,
digamos, um sensor de temperatura, que pode ser usado em qualquer chip para o
qual exista uma implementação dos traits do [`embedded-hal`], simplesmente
escrevendo o driver de tal forma que ele dependa apenas dos traits do
[`embedded-hal`]. Drivers escritos dessa forma são chamados de plataforma
agnósticos e, felizmente para nós, a maioria dos drivers no crates.io são
realmente plataforma agnósticos.

[`embedded-hal`]: https://crates.io/crates/embedded-hal

<!-- ## Further reading -->

## Leitura adicional

<!-- If you want to learn more about these levels of abstraction, Franz Skarman,
a.k.a. [TheZoq2], held a talk about this topic during Oxidize 2020, called [An
Overview of the Embedded Rust Ecosystem]. -->

Se você quiser aprender mais sobre esses níveis de abstração, Franz Skarman,
também conhecido como [TheZoq2], ministrou uma palestra sobre esse assunto
durante o Oxidize 2020, chamada
[Uma Visão Geral do Ecossistema de Rust para Sistemas Embarcados].

[TheZoq2]: https://github.com/TheZoq2/

<!-- [An Overview of the Embedded Rust Ecosystem]: https://www.youtube.com/watch?v=vLYit_HHPaY -->

[Uma Visão Geral do Ecossistema de Rust para Sistemas Embarcados]: https://www.youtube.com/watch?v=vLYit_HHPaY
