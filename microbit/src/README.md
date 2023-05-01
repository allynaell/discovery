# Discovery

<!-- > Discover the world of microcontrollers through [Rust]!

[Rust]: https://www.rust-lang.org/ -->

> Descubra o mundo dos microcontroladores através do [Rust]!

[Rust]: https://www.rust-lang.org/

<!-- This book is an introductory course on microcontroller-based embedded systems that uses Rust as the
teaching language rather than the usual C/C++. -->

Este livro é um curso introdutório sobre sistemas embarcados baseados em
microcontroladores que usa Rust como linguagem de ensino ao invés do C/C++
usual.

<!-- ## Scope -->

## Escopo

<!-- The following topics will be covered (eventually, I hope): -->

Os seguintes tópicos serão abordados (eventualmente, espero):

<!-- - How to write, build, flash and debug an "embedded" (Rust) program. -->

- Como escrever, construir, atualizar e depurar um programa (Rust) "embarcado".

<!-- - Functionality ("peripherals") commonly found in microcontrollers: Digital
  input and output, Pulse Width Modulation (PWM), Analog to Digital Converters
  (ADC), common communication protocols like Serial, I2C and SPI, etc. -->

- Funcionalidades ("periféricos") comumente encontradas em microcontroladores:
  entrada e saída digital, modulação por largura de pulso (PWM), conversores
  analógicos para digitais (ADC), protocolos de comunicação comuns como Serial,
  I2C e SPI, etc.

<!-- - Multitasking concepts: cooperative vs preemptive multitasking, interrupts,
  schedulers, etc. -->

- Conceitos de multitarefa: multitarefa cooperativa vs preemptiva, interrupções,
  escalonadores, etc.

<!-- - Control systems concepts: sensors, calibration, digital filters, actuators,
  open loop control, closed loop control, etc. -->

- Conceitos de sistemas de controle: sensores, calibração, filtros digitais,
  atuadores, controle de malha aberta, controle de malha fechada, etc.

<!-- ## Approach -->

## Abordagem

<!-- - Beginner friendly. No previous experience with microcontrollers or embedded
  systems is required. -->

- Amigável para iniciantes. Nenhuma experiência anterior com microcontroladores
  ou sistemas embarcados é necessária.

<!-- - Hands on. Plenty of exercises to put the theory into practice. _You_ will be
  doing most of the work here. -->

- Mão na massa. Muitos exercícios para colocar a teoria em prática. _Você_ fará
  a maior parte do trabalho aqui.

<!-- - Tool centered. We'll make plenty use of tooling to ease development. "Real"
  debugging, with GDB, and logging will be introduced early on. Using LEDs as a
  debugging mechanism has no place here. -->

- Centrado na ferramenta. Faremos bastante uso de ferramentas para facilitar o
  desenvolvimento. A depuração "real", com GDB, e o log serão introduzidos logo
  no início. Usar LEDs como um mecanismo de depuração não tem lugar aqui.

<!-- ## Non-goals -->

## Fora do escopo

<!-- What's out of scope for this book: -->

O que está fora do escopo deste livro:

<!-- - Teaching Rust. There's plenty of material on that topic already. We'll focus
  on microcontrollers and embedded systems. -->

- Ensinar Rust. Já existe bastante material sobre esse assunto. Vamos nos
  concentrar em microcontroladores e sistemas embarcados.

<!-- - Being a comprehensive text about electric circuit theory or electronics. We'll
  just cover the minimum required to understand how some devices work. -->

- Ser um texto abrangente sobre teoria de circuitos elétricos ou eletrônica.
  Abordaremos apenas o mínimo necessário para entender como alguns dispositivos
  funcionam.

<!-- - Covering details such as linker scripts and the boot process. For example,
  we'll use existing tools to help get your code onto your board, but not go
  into detail about how those tools work. -->

- Cobrir detalhes como linker scripts e o processo de boot. Por exemplo,
  usaremos as ferramentas existentes para ajudar a inserir seu código em sua
  placa, mas não entraremos em detalhes sobre como essas ferramentas funcionam.

<!-- Also I don't intend to port this material to other development boards; this book
will make exclusive use of the micro:bit development board. -->

Também não pretendo portar este material para outras placas de desenvolvimento;
este livro fará uso exclusivo da placa de desenvolvimento micro:bit.

<!-- ## Reporting problems -->

## Relatando problemas

<!-- The source of this book is in [this repository]. If you encounter any typo or
problem with the code report it on the [issue tracker]. -->

A fonte original deste livro está [neste repositório]. Se você encontrar algum
problema com o código, informe-o no [rastreador de issues].

[neste repositório]: https://github.com/rust-embedded/discovery
[rastreador de issues]: https://github.com/rust-embedded/discovery/issues

Já o repositório da tradução para português está [aqui]. Caso você encontre
algum erro de digitação ou qualquer outro problema relacionado a ela, informe-o
na [seção de issues].

[aqui]: https://github.com/allynaell/discovery
[seção de issues]: https://github.com/allynaell/discovery/issues

<!-- ## Other embedded Rust resources -->

## Outros materiais sobre Rust para embarcados:

<!-- This Discovery book is just one of several embedded Rust resources provided by
the [Embedded Working Group]. The full selection can be found at
[The Embedded Rust Bookshelf]. This includes the list of
[Frequently Asked Questions]. -->

Este livro de Discovery é apenas um de vários materiais acerca de Rust para
embarcados fornecidos pelo [Embedded Working Group]. A seleção completa pode ser
encontrada em [The Embedded Rust Bookshelf]. Isso inclui a lista de
[perguntas frequentes].

[Embedded Working Group]: https://github.com/rust-embedded/wg
[The Embedded Rust Bookshelf]: https://docs.rust-embedded.org
[perguntas frequentes]: https://docs.rust-embedded.org/faq.html
