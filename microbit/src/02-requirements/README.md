<!-- # Hardware/knowledge requirements -->

# Requisitos de hardware/conhecimento

<!-- The primary knowledge requirement to read this book is to know _some_ Rust. It's
hard for me to quantify _some_ but at least I can tell you that you don't need
to fully grok generics, but you do need to know how to _use_ closures. You also
need to be familiar with the idioms of the [2018 edition], in particular with
the fact that `extern crate` is not necessary in the 2018 edition. -->

O principal requisito de conhecimento para ler este livro é saber _um pouco_ de
Rust. É difícil para mim quantificar _um pouco_, mas pelo menos posso dizer que
você não precisa entender totalmente os genéricos, mas precisa saber como _usar_
closures. Você também precisa estar familiarizado com os idiomas da
[edição de 2018], em particular com o fato de que `extern crate` não é
necessário nela.

<!-- [2018 edition]: https://rust-lang-nursery.github.io/edition-guide/ -->

[edição de 2018]: https://rust-lang-nursery.github.io/edition-guide/

<!-- Also, to follow this material you'll need the following hardware: -->

Além disso, para acompanhar este material, você precisará do seguinte hardware:

<!-- - A [micro:bit v2] board, alternatively a [micro:bit v1.5] board, the book will
  refer to the v1.5 as just v1. -->

- Uma placa [micro:bit v2], alternativamente uma placa [micro:bit v1.5]. O livro
  se referirá à v1.5 apenas como v1.

[micro:bit v2]: https://tech.microbit.org/hardware/
[micro:bit v1.5]: https://tech.microbit.org/hardware/1-5-revision/

<!-- (You can purchase this board from several [electronics][0] [suppliers][1]) -->

(Você pode comprar essa placa em diversos [fornecedores][1] [de eletrônicos][0])

[0]: https://microbit.org/buy/
[1]: https://www.mouser.com/microbit/_/N-aez3t?P=1y8um0l

<p align="center">
<img title="micro:bit" src="../assets/microbit-v2.jpg">
</p>

<!-- > **NOTE** This is an image of a micro:bit v2, the front of the v1 looks
> slightly different -->

> **NOTA** Esta é uma imagem de uma placa micro:bit v2, a parte da frente da v1
> parece um pouco diferente.

<!-- - One micro-B USB cable, required to make the micro:bit board work. Make sure
  that the cable supports data transfer as some cables only support charging
  devices. -->

- Um cabo micro-B USB, necessário para fazer a placa micro:bit funcionar.
  Certifique-se de que o cabo suporta transferência de dados, pois alguns cabos
  suportam apenas o carregamento de dispositivos.

<p align="center">
<img title="Cabo micro-B USB" src="../assets/usb-cable.jpg">
</p>

<!-- > **NOTE** You may already have a cable like this, as some micro:bit kits ship
> with such cables. Some USB cables used to charge mobile devices may also work,
> if they are micro-B and have the capability to transmit data. -->

> **NOTA** Talvez você já tenha um cabo como este, pois alguns kits de micro:bit
> são enviados com cabos desse tipo. Alguns cabos USB usados para carregar
> dispositivos móveis também podem funcionar, se forem micro-B e tiverem a
> capacidade de transmitir dados.

<!-- > **FAQ**: Wait, why do I need this specific hardware? -->

> **Perguntas frequentes**: Espere, por que preciso desse hardware específico?

<!-- It makes my life and yours much easier. -->

Isso torna minha vida e a sua muito mais fácil.

<!-- The material is much, much more approachable if we don't have to worry about
hardware differences. Trust me on this one. -->

O material é muito, muito mais acessível se não tivermos que nos preocupar com
diferenças de hardware. Confie em mim neste caso.

<!-- > **FAQ**: Can I follow this material with a different development board? -->

> **Perguntas frequentes**: Posso seguir este material com uma placa de
> desenvolvimento diferente?

<!-- Maybe? It depends mainly on two things: your previous experience with
microcontrollers and/or whether a high level crate already exists, like the
[`nrf52-hal`], for your development board somewhere. You can look through the
[Awesome Embedded Rust HAL list] for your microcontroller, if you intend to use
a different one. -->

Talvez? Isso depende principalmente de duas coisas: sua experiência anterior com
microcontroladores e/ou se já existe um crate de alto nível, como o
[`nrf52-hal`], para sua placa de desenvolvimento em algum lugar. Você pode
consultar a lista [Awesome Embedded Rust HAL] para o seu microcontrolador, caso
pretenda usar um diferente.

[`nrf52-hal`]: https://docs.rs/nrf52-hal
[Awesome Embedded Rust HAL]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates

<!-- With a different development board, this text would lose most if not all its
beginner friendliness and "easy to follow"-ness, IMO. -->

Com uma placa de desenvolvimento diferente, este texto perderia grande parte,
senão toda, de sua facilidade para iniciantes e sua capacidade de ser "fácil de
seguir", em minha opinião.

<!-- If you have a different development board and you don't consider yourself a
total beginner, you are better off starting with the [quickstart] project
template. -->

Se você tiver uma placa de desenvolvimento diferente e não se considerar um
iniciante total, é melhor começar com o template de projeto para
[início rápido].

[início rápido]: https://rust-embedded.github.io/cortex-m-quickstart/cortex_m_quickstart/
