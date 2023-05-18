<!-- # Nordic nRF51822 (the "nRF51", micro:bit v1) -->

# Nordic nRF51822 (o "nRF51", micro:bit v1)

<!-- Our MCU has 48 tiny metal **pins** sitting right underneath it (it's a so called
[QFN48] chip). These pins are connected to **traces**, the little "roads" that
act as the wires connecting components together on the board. The MCU can
dynamically alter the electrical properties of the pins. This works similar to a
light switch altering how electrical current flows through a circuit. By
enabling or disabling electrical current to flow through a specific pin, an LED
attached to that pin (via the traces) can be turned on and off. -->

Nosso MCU possui 48 pequenos **pinos** metálicos situados logo abaixo dele (é um
chip chamado [QFN48]). Esses pinos estão conectados a **trilhas**, as pequenas
"estradas" que funcionam como fios conectando os componentes na placa. O MCU
pode alterar dinamicamente as propriedades elétricas dos pinos. Isso funciona de
forma semelhante a um interruptor de luz, alterando como a corrente elétrica
flui por um circuito. Ao habilitar ou desabilitar o fluxo de corrente elétrica
por um pino específico, um LED conectado a esse pino (por meio das trilhas) pode
ser ligado e desligado.

<!-- Each manufacturer uses a different part numbering scheme, but many will allow
you to determine information about a component simply by looking at the part
number. Looking at our MCU's part number (`N51822 QFAAH3 1951LN`, you probably
cannot see it with your bare eye, but it is on the chip), the `n` at the front
hints to us that this is a part manufactured by [Nordic Semiconductor]. Looking
up the part number on their website we quickly find the [product page]. There we
learn that our chip's main marketing point is that it is a "Bluetooth Low Energy
and 2.4 GHz SoC" (SoC being short for "System on a Chip"), which explains the RF
in the product name since RF is short for radio frequency. If we search through
the documentation of the chip linked on the [product page] for a bit we find the
[product specification] which contains chapter 10 "Ordering Information"
dedicated to explaining the weird chip naming. Here we learn that: -->

Cada fabricante utiliza um esquema de numeração de peças diferente, mas muitos
permitem que você determine as informações sobre um componente simplesmente
olhando para o número da peça. Analisando o número da peça do nosso MCU
(`N51822 QFAAH3 1951LN`, você provavelmente não consegue vê-lo a olho nu, mas
está no chip), o `n` na frente nos indica que essa é uma peça fabricada pela
[Nordic Semiconductor]. Pesquisando o número da peça em seu site, encontramos
rapidamente a [página do produto]. Lá descobrimos que o principal destaque do
nosso chip é que ele é um "Bluetooth Low Energy and 2.4 GHz SoC" (SoC sendo a
sigla para "System on a Chip"), o que explica o "RF" no nome do produto, já que
RF é a sigla para radiofrequência. Se pesquisarmos um pouco na documentação do
chip vinculada na [página do produto], encontraremos a
[especificação do produto] que contém o capítulo 10 "Informações de Pedido"
dedicado a explicar a estranha nomenclatura do chip. Aqui aprendemos que:

[QFN48]: https://en.wikipedia.org/wiki/Flat_no-leads_package
[Nordic Semiconductor]: https://www.nordicsemi.com/

<!-- [product page]: https://www.nordicsemi.com/products/nrf51822 -->

[página do produto]: https://www.nordicsemi.com/products/nrf51822

<!-- [product specification]: https://infocenter.nordicsemi.com/pdf/nRF51822_PS_v3.3.pdf -->

[especificação do produto]: https://infocenter.nordicsemi.com/pdf/nRF51822_PS_v3.3.pdf

<!-- - The `N51` is the MCU's series, indicating that there are other `nRF51` MCUs -->

- A `N51` é a série do MCU, indicando que existem outros MCUs `nRF51`.

<!-- - The `822` is the part code -->

- O `822` é o código da peça.

<!-- - The `QF` is the package code, in this case short for `QFN48` -->

- O `QF` é o código do pacote, neste caso, abreviado para `QFN48`.

<!-- - The `AA` is the variant code, indicating how much RAM and flash memory the MCU
  has, in our case 256 kilobyte flash and 16 kilobyte RAM -->

- O `AA` é o código de variante, indicando a quantidade de RAM e memória flash
  que o MCU possui, no nosso caso, 256 kilobytes de flash e 16 kilobytes de RAM.

<!-- - The `H3` is the build code, indicating the hardware version (`H`) as well as
  the product configuration (`3`) -->

- O `H3` é o código de construção, indicando a versão do hardware (`H`) e a
  configuração do produto (`3`).

<!-- - The `1951LN` is a tracking code, hence it might differ on your chip -->

- O `1951LN` é um código de rastreamento, por isso pode ser diferente no seu
  chip.

<!-- The product specification does of course contain a lot more useful information
about the chip, for example that it is based on an ARM® Cortex™-M0 32-bit
processor. -->

A especificação do produto, é claro, contém muitas outras informações úteis
sobre o chip, por exemplo, que ele é baseado em um processador ARM® Cortex™-M0
de 32 bits.

### Arm? Cortex-M0?

<!-- If our chip is manufactured by Nordic, then who is Arm? And if our chip is the
nRF51822, what is the Cortex-M0? -->

Se nosso chip for fabricado pela Nordic, então quem é a Arm? E se nosso chip é o
nRF51822, o que é o Cortex-M0?

<!-- You might be surprised to hear that while "Arm-based" chips are quite popular,
the company behind the "Arm" trademark ([Arm Holdings]) doesn't actually
manufacture chips for purchase. Instead, their primary business model is to just
_design_ parts of chips. They will then license those designs to manufacturers,
who will in turn implement the designs (perhaps with some of their own tweaks)
in the form of physical hardware that can then be sold. Arm's strategy here is
different from companies like Intel, which both designs _and_ manufactures their
chips. -->

Você pode se surpreender ao saber que, embora os chips "baseados na Arm" sejam
bastante populares, a empresa por trás da marca registrada "Arm"
([Arm Holdings]) na verdade não fabrica chips para venda. Em vez disso, seu
principal modelo de negócios é apenas _projetar_ partes dos chips. Eles
licenciam esses designs para fabricantes, que, por sua vez, os implementam
(talvez com algumas modificações próprias) na forma de hardware físico que pode
ser vendido. A estratégia da Arm aqui é diferente de empresas como a Intel, que
projetam _e_ fabricam seus próprios chips.

<!-- Arm licenses a bunch of different designs. Their "Cortex-M" family of designs
are mainly used as the core in microcontrollers. For example, the Cortex-M0 (the
core our chip is based on) is designed for low cost and low power usage. The
Cortex-M7 is higher cost, but with more features and performance. -->

A Arm licencia diversos designs diferentes. Sua família de designs "Cortex-M" é
principalmente usada como núcleo em microcontroladores. Por exemplo, o Cortex-M0
(o núcleo em que nosso chip é baseado) é projetado para baixo custo e baixo
consumo de energia. O Cortex-M7 tem custo mais alto, mas com mais recursos e
desempenho.

<!-- Luckily, you don't need to know too much about different types of processors
or Cortex designs for the sake of this book. However, you are hopefully now a
bit more knowledgeable about the terminology of your device. While you are
working specifically with an nRF51822, you might find yourself reading
documentation and using tools for Cortex-M-based chips, as the nRF51822 is
based on a Cortex-M design. -->

Felizmente, você não precisa saber muito sobre diferentes tipos de processadores
ou designs Cortex para fins deste livro. No entanto, agora você tem um pouco
mais de conhecimento sobre a terminologia do seu dispositivo. Embora você esteja
trabalhando especificamente com um nRF51822, pode encontrar-se lendo
documentação e usando ferramentas para chips baseados em Cortex-M, pois o
nRF51822 é baseado em um design Cortex-M.

[Arm Holdings]: https://www.arm.com/
