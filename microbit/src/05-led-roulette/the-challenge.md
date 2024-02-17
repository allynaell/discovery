<!-- # The challenge -->

# Desafio

<!-- You are now well armed to face a challenge! Your task will be to implement the application I showed
you at the beginning of this chapter. -->

Agora você está bem armado para enfrentar um desafio! Sua tarefa será implementar a aplicação que mostrei a você no início deste capítulo.

<p align="center">
<video src="../assets/roulette_fast.mp4" loop autoplay>
</p>

<!-- If you can't exactly see what's happening here it is in a much slower version: -->

Se você não consegue ver exatamente o que está acontecendo, aqui está em uma versão muito mais lenta:

<p align="center">
<video src="../assets/roulette_slow.mp4" loop autoplay>
</p>

<!-- Since working with the LED pins separately is quite annoying
(especially if you have to use basically all of them like here)
you can use the display API provided by the BSP. It works like this: -->

Como trabalhar com os pinos de LED separadamente é bastante incômodo (especialmente se você tiver que usar basicamente todos eles, como aqui) você pode usar a API de exibição fornecida pelo BSP. Ela funciona da seguinte forma:

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let light_it_all = [
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
    ];

    loop {
        // Exibe light_it_all por 1000ms
        display.show(&mut timer, light_it_all, 1000);
        // Limpa a exibição novamente
        display.clear();
        timer.delay_ms(1000_u32);
    }
}
```

<!-- Equipped with this API your task basically boils down to just having
to calculate the proper image matrix and passing it into the BSP. -->

Equipado com essa API, sua tarefa basicamente se resume a calcular a matriz de imagem adequada e passá-la para o BSP.
