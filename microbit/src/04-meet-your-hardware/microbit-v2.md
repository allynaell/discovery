<!-- # Nordic nRF52833 (the "nRF52", micro:bit v2) -->

# Nordic nRF52833 (o "nRF52", micro:bit v2)

<!-- Our MCU has 73 tiny metal **pins** sitting right underneath it (it's a so called [aQFN73] chip).
These pins are connected to **traces**, the little "roads" that act as the wires connecting components
together on the board. The MCU can dynamically alter the electrical properties
of the pins. This works similar to a light switch altering how electrical
current flows through a circuit. By enabling or disabling electrical current to
flow through a specific pin, an LED attached to that pin (via the traces) can
be turned on and off. -->

Nosso MCU possui 73 pequenos **pinos** metálicos situados logo abaixo dele (é um
chip chamado [aQFN73]). Esses pinos estão conectados a **trilhas**, as pequenas
"estradas" que funcionam como fios conectando os componentes na placa. O MCU
pode alterar dinamicamente as propriedades elétricas dos pinos. Isso funciona de
forma semelhante a um interruptor de luz, alterando como a corrente elétrica
flui por um circuito. Ao habilitar ou desabilitar o fluxo de corrente elétrica
por um pino específico, um LED conectado a esse pino (por meio das trilhas) pode
ser ligado e desligado.

<!-- Each manufacturer uses a different part numbering scheme, but many will allow
you to determine information about a component simply by looking at the part
number. Looking at our MCU's part number (`N52833 QIAAA0 2024AL`, you probably
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
(`N52833 QIAAA0 2024AL`, você provavelmente não consegue vê-lo a olho nu, mas
está no chip), o `n` na frente nos indica que essa é uma peça fabricada pela
[Nordic Semiconductor]. Pesquisando o número da peça em seu site, encontramos
rapidamente a [página do produto]. Lá descobrimos que o principal destaque do
nosso chip é que ele é um "Bluetooth Low Energy and 2.4 GHz SoC" (SoC sendo a
sigla para "System on a Chip"), o que explica o "RF" no nome do produto, já que
RF é a sigla para radiofrequência. Se pesquisarmos um pouco na documentação do
chip vinculada na [página do produto], encontraremos a
[especificação do produto] que contém o capítulo 10 "Informações de Pedido"
dedicado a explicar a estranha nomenclatura do chip. Aqui aprendemos que:

[aQFN73]: https://en.wikipedia.org/wiki/Flat_no-leads_package
[Nordic Semiconductor]: https://www.nordicsemi.com/

<!-- [product page]: https://www.nordicsemi.com/products/nrf52833 -->

[página do produto]: https://www.nordicsemi.com/products/nrf52833

<!-- [product specification]: https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.3.pdf -->

[especificação do produto]: https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.3.pdf

<!-- - The `N52` is the MCU's series, indicating that there are other `nRF52` MCUs
- The `833` is the part code
- The `QI` is the package code, short for `aQFN73`
- The `AA` is the variant code, indicating how much RAM and flash memory the MCU
  has, in our case 512 kilobyte flash and 128 kilobyte RAM
- The `A0` is the build code, indicating the hardware version (`A`) as well as
  the product configuration (`0`)
- The `2024AL` is a tracking code, hence it might differ on your chip -->

- O `N52` é a série do MCU, indicando que existem outros MCUs `nRF52`.
- O `833` é o código da peça.
- O `QI` é o código do pacote, abreviação de `aQFN73`.
- O `AA` é o código da variante, indicando quanta RAM e memória flash o MCU
  possui, no nosso caso, 512 kilobytes de flash e 128 kilobytes de RAM.
- O `A0` é o código de construção, indicando a versão do hardware (`A`) e a
  configuração do produto (`0`).
- O `2024AL` é um código de rastreamento, portanto pode ser diferente no seu
  chip.

<!-- The product specification does of course contain a lot more useful information
about the chip, for example that it is based on an ARM® Cortex™-M4 32-bit
processor. -->

A especificação do produto, é claro, contém muitas outras informações úteis
sobre o chip, por exemplo, que ele é baseado em um processador ARM® Cortex™-M4
de 32 bits.

## Arm? Cortex-M4?

<!-- If our chip is manufactured by Nordic, then who is Arm? And if our chip is the
nRF52833, what is the Cortex-M4? -->

Se nosso chip for fabricado pela Nordic, então quem é a Arm? E se nosso chip é o
nRF52833, o que é o Cortex-M4?

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
are mainly used as the core in microcontrollers. For example, the Cortex-M4 (the
core our chip is based on) is designed for low cost and low power usage. The
Cortex-M7 is higher cost, but with more features and performance. -->

A Arm licencia diversos designs diferentes. Sua família de designs "Cortex-M" é
principalmente usada como núcleo em microcontroladores. Por exemplo, o Cortex-M4
(o núcleo em que nosso chip é baseado) é projetado para baixo custo e baixo
consumo de energia. O Cortex-M7 tem custo mais alto, mas com mais recursos e
desempenho.

<!-- Luckily, you don't need to know too much about different types of processors or
Cortex designs for the sake of this book. However, you are hopefully now a bit
more knowledgeable about the terminology of your device. While you are working
specifically with an nRF52833, you might find yourself reading documentation and
using tools for Cortex-M-based chips, as the nRF52833 is based on a Cortex-M
design. -->

Felizmente, você não precisa saber muito sobre diferentes tipos de processadores
ou designs Cortex para fins deste livro. No entanto, agora você tem um pouco
mais de conhecimento sobre a terminologia do seu dispositivo. Embora você esteja
trabalhando especificamente com um nRF52833, pode encontrar-se lendo
documentação e usando ferramentas para chips baseados em Cortex-M, pois o
nRF52833 é baseado em um design Cortex-M.

[Arm Holdings]: https://www.arm.com/
