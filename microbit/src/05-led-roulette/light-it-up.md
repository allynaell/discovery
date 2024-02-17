<!-- # Light it up -->

# Acendendo

## embedded-hal

<!-- In this chapter we are going to make one of the many LEDs on the back of the micro:bit light up since this is
basically the "Hello World" of embedded programming. In order to get this task done we will use one of the traits
provided by `embedded-hal`, specifically the [`OutputPin`] trait which allows us to turn a pin on or off. -->

Neste capítulo, vamos fazer com que um dos muitos LEDs na parte de trás do
micro:bit acenda, já que isso é basicamente o "Hello World" da programação
embarcada. Para realizar essa tarefa, vamos usar um dos recursos fornecidos pelo
`embedded-hal`, especificamente o trait [`OutputPin`], que nos permite ligar ou
desligar um pino.

[`OutputPin`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/digital/v2/trait.OutputPin.html

<!-- ## The micro:bit LEDs -->

## Os LEDs do micro:bit

<!-- On the back of the micro:bit you can see a 5x5 square of LEDs, usually called an
LED matrix. This matrix alignment is used so that instead of having to use 25
separate pins to drive every single one of the LEDs, we can just use 10 (5+5)
pins in order to control which column and which row of our matrix lights up. -->

Na parte de trás do micro:bit, você pode ver um quadrado de LEDs de 5x5,
geralmente chamado de matriz de LEDs. Essa disposição em matriz é usada para
que, em vez de precisarmos usar 25 pinos separados para controlar cada um dos
LEDs, podemos usar apenas 10 (5+5) pinos para controlar qual coluna e qual linha
de nossa matriz se acende.

<!-- > **NOTE** that the micro:bit v1 team implemented this a little differently.
> Their [schematic page] says that it is actually implemented as a 3x9 matrix
> but a few columns simply remain unused. -->

> **NOTE** que a equipe do micro:bit v1 implementou isso de forma um pouco
> diferente. A [página de esquema] deles indica que na verdade é implementado
> como uma matriz de 3x9, mas algumas colunas simplesmente permanecem não
> utilizadas.

<!-- Usually in order to determine which specific pins we have to control in order to
light a specific LED up we would now have to read the [micro:bit v2 schematic]
or the [micro:bit v1 schematic] respectively. Luckily for us though we can use
the aforementioned micro:bit BSP which abstracts all of this nicely away from
us. -->

Normalmente, para determinar quais pinos específicos devemos controlar para
acender um LED específico, agora teríamos que ler o [esquema do micro:bit v2] ou
o [esquema do micro:bit v1], respectivamente. Felizmente para nós, no entanto,
podemos usar o BSP do micro:bit mencionado anteriormente, que abstrai todas
essas informações de forma conveniente para nós.

<!-- [schematic page]: https://tech.microbit.org/hardware/schematic/ -->

[página de esquema]: https://tech.microbit.org/hardware/schematic/

<!-- [micro:bit v2 schematic]: https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.00/MicroBit_V2.0.0_S_schematic.PDF -->

[esquema do micro:bit v2]: https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.00/MicroBit_V2.0.0_S_schematic.PDF

<!-- [micro:bit v1 schematic]: https://github.com/bbcmicrobit/hardware/blob/master/V1.5/SCH_BBC-Microbit_V1.5.PDF -->

[esquema do micro:bit v1]: https://github.com/bbcmicrobit/hardware/blob/master/V1.5/SCH_BBC-Microbit_V1.5.PDF

<!-- ## Actually lighting it up! -->

## Realmente acendendo!

<!-- The code required to light up an LED in the matrix is actually quite simple but
it requires a bit of setup. First take a look at it and then we can go through
it step by step: -->

O código necessário para acender um LED na matriz é, na verdade, bastante
simples, mas requer um pouco de configuração. Primeiro, dê uma olhada e depois
podemos passar por ele passo a passo:

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::board::Board;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    loop {}
}
```

<!-- The first few lines until the main function just do some basic imports and setup
we already looked at before. However, the main function looks pretty different
to what we have seen up to now. -->

As primeiras linhas até a função principal apenas realizam algumas importações
básicas e configurações que já analisamos anteriormente. No entanto, a função
principal parece bastante diferente do que vimos até agora.

<!-- The first line is related to how most HALs written in Rust work internally. As
discussed before they are built on top of PAC crates which own (in the Rust
sense) all the peripherals of a chip. `let mut board = Board::take().unwrap();`
basically takes all these peripherals from the PAC and binds them to a variable.
In this specific case we are not only working with a HAL but with an entire BSP,
so this also takes ownership of the Rust representation of the other chips on
the board. -->

A primeira linha está relacionada à forma como a maioria das HALs escritas em
Rust funcionam internamente. Como discutido anteriormente, elas são construídas
em cima de crates PAC que possuem (no sentido Rust) todos os periféricos de um
chip. `let mut board = Board::take().unwrap();` basicamente pega todos esses
periféricos do PAC e os vincula a uma variável. Neste caso específico, não
estamos apenas trabalhando com uma HAL, mas com todo um BSP, então isso também
assume a propriedade da representação Rust de outros chips na placa.

<!-- > **NOTE**: If you are wondering why we have to call `unwrap()` here, in theory
> it is possible for `take()` to be called more than once. This would lead to
> the peripherals being represented by two separate variables and thus lots of
> possible confusing behaviour because two variables modify the same resource.
> In order to avoid this, PACs are implemented in a way that it would panic if
> you tried to take the peripherals twice. -->

> **NOTA**: Se você está se perguntando por que tivemos que chamar `unwrap()`
> aqui, teoricamente é possível que `take()` seja chamado mais de uma vez. Isso
> levaria aos periféricos sendo representados por duas variáveis separadas e,
> portanto, muitos comportamentos possivelmente confusos, porque duas variáveis
> modificam o mesmo recurso. Para evitar isso, as PACs são implementadas de tal
> forma que causariam um pânico se você tentasse pegar os periféricos duas
> vezes.

<!-- Now we can light the LED connected to `row1`, `col1` up by setting the `row1`
pin to high (i.e. switching it on). The reason we can leave `col1` set to low is
because of how the LED matrix circuit works. Furthermore, `embedded-hal` is
designed in a way that every operation on hardware can possibly return an error,
even just toggling a pin on or off. Since that is highly unlikely in our case,
we can just `unwrap()` the result. -->

Agora podemos acender o LED conectado a `row1`, `col1` definindo o pino `row1`
como alto (ou seja, ligando-o). A razão pela qual podemos deixar `col1` definido
como baixo é por causa do funcionamento do circuito da matriz de LEDs. Além
disso, `embedded-hal` é projetado de forma que cada operação no hardware possa
retornar possivelmente um erro, mesmo apenas alternando um pino entre ligado e
desligado. Como isso é altamente improvável em nosso caso, podemos apenas
`unwrap()` o resultado.

<!-- ## Testing it -->

## Testando

<!-- Testing our little program is quite simple. First put it into `src/main.rs`.
Afterwards we simply have to run the `cargo embed` command from the last section
again, let it flash and just like before. Then open our GDB and connect to the
GDB stub: -->

Testar nosso pequeno programa é bastante simples. Primeiro, coloque-o em
`src/main.rs`. Em seguida, basta executar o comando `cargo embed` da última
seção novamente, deixe-o gravar e apenas como antes. Em seguida, abra o GDB e
conecte-se ao GDB stub:

```
$ # Seu comando de depuração GDB da última seção
(gdb) target remote :1337
Remote debugging using :1337
cortex_m_rt::Reset () at /home/nix/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.12/src/lib.rs:489
489     pub unsafe extern "C" fn Reset() -> ! {
(gdb)
```

<!-- If we now let the program run via the GDB `continue` command, one of the LEDs on
the back of the micro:bit should light up. -->

Se agora deixarmos o programa rodar usando o comando `continue` do GDB, um dos
LEDs na parte de trás do micro:bit deve acender.
