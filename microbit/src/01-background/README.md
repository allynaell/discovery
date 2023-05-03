<!-- # Background -->

# Contexto

<!-- ## What's a microcontroller? -->

## O que é um microcontrolador?

<!-- A microcontroller is a _system_ on a chip. Whereas your computer is made up of
several discrete components: a processor, RAM, storage, an Ethernet port, etc.;
a microcontroller has all those types of components built into a single "chip"
or package. This makes it possible to build systems with fewer parts. -->

Um microcontrolador é um _sistema_ em um chip. Enquanto um computador é composto
por vários componentes discretos, como um processador, RAM, armazenamento, uma
porta Ethernet, etc., um microcontrolador tem todos esses tipos de componentes
incorporados em um único "chip" ou pacote. Isso torna possível construir
sistemas com menos peças.

<!-- ## What can you do with a microcontroller? -->

## O que você pode fazer com um microcontrolador?

<!-- Lots of things! Microcontrollers are the central part of what are known as
"_embedded_ systems". Embedded systems are everywhere, but you don't usually
notice them. They control the machines that wash your clothes, print your
documents, and cook your food. Embedded systems keep the buildings that you live
and work in at a comfortable temperature, and control the components that make
the vehicles you travel in stop and go. -->

Muitas coisas! Os microcontroladores são a parte central do que são conhecidos
como "sistemas _embarcados_". Os sistemas embarcados estão em toda parte, mas
geralmente você não os percebe. Eles controlam as máquinas que lavam suas
roupas, imprimem seus documentos e preparam sua comida. Os sistemas embarcados
mantêm os prédios onde você vive e trabalha em uma temperatura confortável e
controlam os componentes que fazem os veículos nos quais você viaja pararem e
seguirem em frente.

<!-- Most embedded systems operate without user intervention. Even if they expose a
user interface like a washing machine does; most of their operation is done on
their own. -->

A maioria dos sistemas embarcados opera sem intervenção do usuário. Mesmo que
eles tenham uma interface de usuário, como uma máquina de lavar roupa, a maior
parte de sua operação é feita por conta própria.

<!-- Embedded systems are often used to _control_ a physical process. To make this
possible, they have one or more devices to tell them about the state of the
world ("sensors"), and one or more devices which allow them to change things
("actuators"). For example, a building climate control system might have: -->

Os sistemas embarcados são frequentemente usados para _controlar_ um processo
físico. Para que isso seja possível, eles possuem um ou mais dispositivos que
informam sobre o estado do mundo ("sensores") e um ou mais dispositivos que
permitem fazer alterações ("atuadores"). Por exemplo, um sistema de controle de
clima de um prédio pode ter:

<!-- - Sensors which measure temperature and humidity in various locations. -->

- Sensores que medem temperatura e umidade em vários locais.

<!-- - Actuators which control the speed of fans. -->

- Atuadores que controlam a velocidade dos ventiladores.

<!-- - Actuators which cause heat to be added or removed from the building. -->

- Atuadores que fazem com que o calor seja adicionado ou removido do prédio.

<!-- ## When should I use a microcontroller? -->

## Quando devo usar um microcontrolador?

<!-- Many of the embedded systems listed above could be implemented with a computer
running Linux (for example a "Raspberry Pi"). Why use a microcontroller instead?
Sounds like it might be harder to develop a program. -->

Muitos dos sistemas embarcados listados acima poderiam ser implementados com um
computador executando Linux (por exemplo, um "Raspberry Pi"). Por que usar um
microcontrolador em vez disso? Parece mais difícil para desenvolver um programa.

<!-- Some reasons might include: -->

Alguns motivos podem incluir:

<!-- **Cost.** A microcontroller is much cheaper than a general purpose computer. Not
only is the microcontroller cheaper; it also requires many fewer external
electrical components to operate. This makes Printed Circuit Boards (PCB)
smaller and cheaper to design and manufacture. -->

**Custo.** Um microcontrolador é muito mais barato do que um computador de
propósito geral. Além de ser mais barato, o microcontrolador também requer menos
componentes elétricos externos para funcionar. Isso torna as Placas de Circuito
Impresso (PCB) menores e mais baratas de serem projetadas e fabricadas.

<!-- **Power consumption.** Most microcontrollers consume a fraction of the power of
a full blown processor. For applications which run on batteries, that makes a
huge difference. -->

**Consumo de energia.** A maioria dos microcontroladores consome uma fração da
energia de um processador completo. Para aplicativos que funcionam com baterias,
isso faz uma enorme diferença.

<!-- **Responsiveness.** To accomplish their purpose, some embedded systems must
always react within a limited time interval (e.g. the "anti-lock" braking system
of a car). If the system misses this type of _deadline_, a catastrophic failure
might occur. Such a deadline is called a "hard real time" requirement. An
embedded system which is bound by such a deadline is referred to as a "hard
real-time system". A general purpose computer and OS usually has many software
components which share the computer's processing resources. This makes it harder
to guarantee execution of a program within tight time constraints. -->

**Responsividade.** Para cumprir seu propósito, alguns sistemas embarcados devem
sempre reagir dentro de um intervalo de tempo limitado (por exemplo, o sistema
de freio "antibloqueio" de um carro). Se o sistema perder esse tipo de _prazo_,
uma falha catastrófica pode ocorrer. Esse tipo de prazo é chamado de requisito
de "tempo real rígido". Um sistema embarcado que está vinculado a esse prazo é
chamado de "sistema de tempo real rígido". Um computador de propósito geral e um
sistema operacional geralmente possuem muitos componentes de software que
compartilham os recursos de processamento do computador. Isso torna mais difícil
garantir a execução de um programa dentro de restrições de tempo rigorosas.

<!-- **Reliability.** In systems with fewer components (both hardware and software),
there is less to go wrong! -->

**Confiabilidade.** Em sistemas com menos componentes (tanto de hardware quanto
de software), há menos coisas que podem dar errado!

<!-- ## When should I _not_ use a microcontroller? -->

## Quando _não_ devo usar um microcontrolador?

<!-- Where heavy computations are involved. To keep their power consumption low,
microcontrollers have very limited computational resources available to them.
For example, some microcontrollers don't even have hardware support for floating
point operations. On those devices, performing a simple addition of single
precision numbers can take hundreds of CPU cycles. -->

Onde computações pesadas estão envolvidas. Para manter seu consumo de energia
baixo, os microcontroladores têm recursos computacionais muito limitados. Por
exemplo, alguns microcontroladores nem mesmo possuem suporte de hardware para
operações de ponto flutuante. Nesses dispositivos, realizar uma simples adição
de números de precisão única pode levar centenas de ciclos de CPU.

<!-- ## Why use Rust and not C? -->

## Por que usar Rust e não C?

<!-- Hopefully, I don't need to convince you here as you are probably familiar with
the language differences between Rust and C. One point I do want to bring up is
package management. C lacks an official, widely accepted package management
solution whereas Rust has Cargo. This makes development _much_ easier. And, IMO,
easy package management encourages code reuse because libraries can be easily
integrated into an application which is also a good thing as libraries get more
"battle testing". -->

Espero não precisar convencê-lo aqui, pois você provavelmente está familiarizado
com as diferenças entre as linguagens Rust e C. Um ponto que eu gostaria de
mencionar é o gerenciamento de pacotes. C carece de uma solução oficial e
amplamente aceita de gerenciamento de pacotes, enquanto Rust possui o Cargo.
Isso facilita _muito_ o desenvolvimento. Além disso, na minha opinião, o
gerenciamento fácil de pacotes incentiva a reutilização de código porque as
bibliotecas podem ser facilmente integradas a um aplicativo, o que também é algo
positivo, pois as bibliotecas recebem mais "testes de batalha".

<!-- ## Why should I not use Rust? -->

## Por que não devo usar Rust?

<!-- Or why should I prefer C over Rust? -->

Ou por que eu deveria preferir C em vez de Rust?

<!-- The C ecosystem is way more mature. Off the shelf solutions for several problems
already exist. If you need to control a time sensitive process, you can grab one
of the existing commercial Real Time Operating Systems (RTOS) out there and
solve your problem. There are no commercial, production-grade RTOSes in Rust yet
so you would have to either create one yourself or try one of the ones that are
in development. You can find a list of those in the [Awesome Embedded Rust]
repository. -->

O ecossistema do C é muito mais maduro. Já existem soluções prontas para vários
problemas. Se você precisa controlar um processo sensível ao tempo, pode
adquirir um dos Sistemas Operacionais de Tempo Real (RTOS) comerciais existentes
e resolver seu problema. Ainda não existem Sistemas Operacionais de Tempo Real
em Rust que sejam comerciais e estejam em nível de produção, então você teria
que criar um por conta própria ou experimentar um dos que estão em
desenvolvimento. Você pode encontrar uma lista deles no repositório
[Awesome Embedded Rust].

[Awesome Embedded Rust]: https://github.com/rust-embedded/awesome-embedded-rust#real-time-operating-system-rtos
