[Introdução](README.md)

- [Contexto](01-background/README.md)

- [Requisitos de hardware/conhecimento](02-requirements/README.md)

- [Configurando o ambiente de desenvolvimento](03-setup/README.md)
  - [Linux](03-setup/linux.md)
  - [Windows](03-setup/windows.md)
  - [macOS](03-setup/macos.md)
  - [Verifique a instalação](03-setup/verify.md)

- [Conheça seu hardware](04-meet-your-hardware/README.md)

- [Roleta LED](05-led-roulette/README.md)
  - [Construindo](05-led-roulette/build-it.md)
  - [Gravando](05-led-roulette/flash-it.md)
  - [Depurando](05-led-roulette/debug-it.md)
  - [As abstrações `led` e `delay`](05-led-roulette/the-led-and-delay-abstractions.md)
  - [Desafio](05-led-roulette/the-challenge.md)
  - [Minha solução](05-led-roulette/my-solution.md)

- [Olá, mundo!](06-hello-world/README.md)
  - [`panic!`](06-hello-world/panic.md)

- [Registros](07-registers/README.md)
  - [RTRM](07-registers/rtrm.md)
  - [(des)Otimização](07-registers/optimization.md)
  - [Endereço `0xBAAAAAAD`](07-registers/bad-address.md)
  - [Ação assustadora à distância](07-registers/spooky-action-at-a-distance.md)
  - [Manipulação segura de tipos](07-registers/type-safe-manipulation.md)

- [LEDs, novamente](08-leds-again/README.md)
  - [Energia](08-leds-again/power.md)
  - [Configuração](08-leds-again/configuration.md)
  - [Solução](08-leds-again/the-solution.md)

- [Relógios e temporizadores](09-clocks-and-timers/README.md)
  - [Delays com o loop `for`](09-clocks-and-timers/for-loop-delays.md)
  - [NOP](09-clocks-and-timers/nop.md)
  - [Temporizador de disparo único](09-clocks-and-timers/one-shot-timer.md)
  - [Inicialização](09-clocks-and-timers/initialization.md)
  - [Espera ocupada](09-clocks-and-timers/busy-waiting.md)
  - [Juntando tudo](09-clocks-and-timers/putting-it-all-together.md)

- [Comunicação serial](10-serial-communication/README.md)
  - [Ferramentas para \*nix](10-serial-communication/nix-tooling.md)
  - [Ferramentas para Windows](10-serial-communication/windows-tooling.md)
  - [Loopbacks](10-serial-communication/loopbacks.md)

- [USART](11-usart/README.md)
  - [Enviando um único byte](11-usart/send-a-single-byte.md)
  - [Enviando uma string](11-usart/send-a-string.md)
  - [Transbordamento de dados](11-usart/buffer-overrun.md)
  - [`uprintln!`](11-usart/uprintln.md)
  - [Recebendo um único byte](11-usart/receive-a-single-byte.md)
  - [Servidor de eco](11-usart/echo-server.md)
  - [Invertendo uma string](11-usart/reverse-a-string.md)
  - [Minha solução](11-usart/my-solution.md)

- [Configuração do Bluetooth](12-bluetooth-setup/README.md)
  - [Linux](12-bluetooth-setup/linux.md)
  - [Loopback](12-bluetooth-setup/loopback.md)
  - [Comandos AT](12-bluetooth-setup/at-commands.md)

- [Serial via Bluetooth](13-serial-over-bluetooth/README.md)
- [I2C](14-i2c/README.md)
  - [O protocolo geral](14-i2c/the-general-protocol.md)
  - [LSM303DLHC](14-i2c/lsm303dlhc.md)
  - [Lendo um único registro](14-i2c/read-a-single-register.md)
  - [Solução](14-i2c/the-solution.md)
  - [Lendo vários registros](14-i2c/read-several-registers.md)

- [Bússola de LED](15-led-compass/README.md)
  - [Abordagem 1](15-led-compasentativa-1.md)
  - [Solução 1](15-led-compass/solution-1.md)
  - [Abordagem 2](15-led-compass/take-2.md)
  - [Solução 2](15-led-compass/solution-2.md)
  - [Magnitude](15-led-compass/magnitude.md)
  - [Calibração](15-led-compass/calibration.md)

- [Medidor de soco](16-punch-o-meter/README.md)
  - [A gravidade está para cima?](16-punch-o-meter/gravity-is-up.md)
  - [Desafio](16-punch-o-meter/the-challenge.md)
  - [Minha solução](16-punch-o-meter/my-solution.md)

- [O que resta para você explorar](explore.md)

---

[Solucionando problemas gerais](appendix/1-general-troubleshooting/README.md)

[Como usar o GDB](appendix/2-how-to-use-gdb/README.md)

<!-- - [Async IO: The future](17-async-io-the-future/README.md) -->
<!--     - [Timer](17-async-io-the-future/timer.md) -->
<!--     - [Serial](17-async-io-the-future/serial.md) -->
<!--     - [The challenge](17-async-io-the-future/the-challenge.md) -->
<!--     - [My solution](17-async-io-the-future/my-solution.md) -->
<!--     - [Another challenge](17-async-io-the-future/another-challenge.md) -->
<!--     - [My other solution](17-async-io-the-future/my-other-solution.md) -->
<!--     - [More challenges](17-async-io-the-future/more-challenges.md) -->

---
