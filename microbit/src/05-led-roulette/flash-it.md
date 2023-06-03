<!-- # Flash it -->

# Gravando

<!-- Flashing is the process of moving our program into the microcontroller's (persistent) memory. Once
flashed, the microcontroller will execute the flashed program every time it is powered on. -->

Flashing é o processo de transferir nosso programa para a memória (persistente)
do microcontrolador. Uma vez programado, o microcontrolador executará o programa
gravado sempre que for ligado.

<!-- In this case, our `led-roulette` program will be the _only_ program in the
microcontroller memory. By this I mean that there's nothing else running on the
microcontroller: no OS, no "daemon", nothing. `led-roulette` has full control
over the device. -->

Neste caso, nosso programa `led-roulette` será o _único_ programa na memória do
microcontrolador. Com isso, quero dizer que não há mais nada sendo executado no
microcontrolador: nenhum sistema operacional, nenhum "daemon", nada. O
`led-roulette` tem controle total sobre o dispositivo.

<!-- Flashing the binary itself is quite simple thanks to `cargo embed`. -->

Gravar o binário em si é bastante simples graças ao `cargo embed`.

<!-- Before executing that command though, let's look into what it actually does. If
you look at the side of your micro:bit with the USB connector facing upwards you
will notice, that there are actually 2 black squares on there (on the micro:bit
v2 there is a third and biggest one, which is a speaker), one is our MCU we
already talked about but what purpose does the other one serve? The other chip
has 3 main purposes: -->

Antes de executar esse comando, vamos ver o que ele realmente faz. Se você olhar
para o lado do seu micro:bit com o conector USB virado para cima, você perceberá
que há na verdade 2 quadrados pretos lá (no micro:bit v2 há um terceiro e maior,
que é um alto-falante), um é o nosso MCU que já discutimos, mas para que serve o
outro? O outro chip tem 3 propósitos principais:

<!-- 1. Provide power from the USB connector to our MCU -->

1. Fornecer energia do conector USB para o nosso MCU.

<!-- 2. Provide a serial to USB bridge for our MCU (we will look into that in a later
   chapter) -->

2. Fornecer uma ponte serial para USB para o nosso MCU (vamos falar sobre isso
   em um capítulo posterior).

<!-- 3. Being a programmer/debugger (this is the relevant purpose for now) -->

3. Ser um programador/depurador (esse é o propósito relevante para agora).

<!-- Basically this chip acts as a bridge between our computer (to which it is
connected via USB) and the MCU (to which it is connected via traces and
communicates with using the SWD protocol). This bridge enables us to flash new
binaries on to the MCU, inspect its state via a debugger and other things. -->

Basicamente, esse chip atua como uma ponte entre nosso computador (ao qual está
conectado via USB) e o MCU (ao qual está conectado por trilhas e comunica-se
usando o protocolo SWD). Essa ponte nos permite programar novos binários no MCU,
inspecionar seu estado por meio de um depurador e realizar outras tarefas.

<!-- So lets flash it! -->

Então vamos gravar!

```console
# Para micro:bit v2
$ cargo embed --features v2 --target thumbv7em-none-eabihf
  (...)
     Erasing sectors ✔ [00:00:00] [####################################################################################################################################################]  2.00KiB/ 2.00KiB @  4.21KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [####################################################################################################################################################]  2.00KiB/ 2.00KiB @  2.71KiB/s (eta 0s )
    Finished flashing in 0.608s

# Para micro:bit v1
$ cargo embed --features v1 --target thumbv6m-none-eabi
  (...)
     Erasing sectors ✔ [00:00:00] [####################################################################################################################################################]  2.00KiB/ 2.00KiB @  4.14KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [####################################################################################################################################################]  2.00KiB/ 2.00KiB @  2.69KiB/s (eta 0s )
    Finished flashing in 0.614s
```

<!-- You will notice that `cargo-embed` blocks after outputting the last line, this
is intended and you should not close it since we need it in this state for the
next step: debugging it! Furthermore, you will have noticed that the
`cargo build` and `cargo embed` are actually passed the same flags, this is
because `cargo embed` actually executes the build and then flashes the resulting
binary on to the chip, hence you can leave out the `cargo build` step in the
future if you want to flash your code right away. -->

Você perceberá que o `cargo-embed` bloqueia após exibir a última linha, isso é
intencional e você não deve fechá-lo, pois precisamos dele nesse estado para a
próxima etapa: depurar! Além disso, você terá percebido que o `cargo build` e o
`cargo embed` na verdade recebem as mesmas flags, isso ocorre porque o
`cargo embed` executa o build e, em seguida, grava o binário resultante no chip,
portanto, você pode pular a etapa de `cargo build` no futuro se quiser gravar
seu código imediatamente.
