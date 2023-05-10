<!-- # Verify the installation -->

# Verifique a instalação

<!-- Let's verify that all the tools were installed correctly. -->

Vamos verificar se todas as ferramentas foram instaladas corretamente.

<!-- ## Linux only -->

## Somente Linux

<!-- ### Verify permissions -->

### Verifique as permissões

<!-- Connect the micro:bit to your computer using a USB cable. -->

Conecte o micro:bit ao seu computador usando um cabo USB.

<!-- The micro:bit should now appear as a USB device (file) in `/dev/bus/usb`. Let's
find out how it got enumerated: -->

O micro:bit agora deve aparecer como um dispositivo USB (arquivo) em
`/dev/bus/usb`. Vamos descobrir como ele foi enumerado:

```console
$ lsusb | grep -i "NXP ARM mbed"
Bus 001 Device 065: ID 0d28:0204 NXP ARM mbed
$ # ^^^        ^^^
```

<!-- In my case, the micro:bit got connected to the bus #1 and got enumerated as the
device #65. This means the file `/dev/bus/usb/001/065` _is_ the micro:bit. Let's
check its permissions: -->

No meu caso, o micro:bit foi conectado ao barramento #1 e foi enumerado como o
dispositivo #65. Isso significa que o arquivo `/dev/bus/usb/001/065` _é_ o
micro:bit. Vamos verificar suas permissões:

```console
$ ls -l /dev/bus/usb/001/065
crw-rw-rw-. 1 root root 189, 64 Sep  5 14:27 /dev/bus/usb/001/065
```

<!-- The permissions should be `crw-rw-rw-`. If it's not ... then check your
[udev rules] and try re-loading them with: -->

As permissões devem ser `crw-rw-rw-`. Se não forem... verifique suas
[regras do udev] e tente recarregá-las com:

[regras do udev]: linux.md#udev-rules

```console
$ sudo udevadm control --reload-rules
```

<!-- # All -->

# Todos

<!-- ## Verifying cargo-embed -->

## Verificando o cargo-embed

<!-- First, connect the micro:bit to your Computer using a USB cable. -->

Primeiro, conecte o micro:bit ao seu computador usando um cabo USB.

<!-- At least an orange LED right next to the USB port of the micro:bit should light
up. Furthermore, if you have never flashed another program on to your micro:bit,
the default program the micro:bit ships with should start blinking the red LEDs
on its back, you can ignore them. -->

Pelo menos um LED laranja ao lado da porta USB do micro:bit deve acender. Além
disso, se você nunca tiver gravado outro programa no seu micro:bit, o programa
padrão que vem com o micro:bit deve começar a piscar os LEDs vermelhos em sua
parte traseira, você pode ignorá-los.

<!-- Next up you will have to modify `Embed.toml` in the `src/03-setup` directory of
the book's source code. In the `default.general` section you will find two
commented out chip variants: -->

Em seguida, você terá que modificar o arquivo `Embed.toml` no diretório
`src/03-setup` do código-fonte do livro. Na seção `default.general`, você
encontrará duas variantes de chip comentadas:

```toml
[default.general]
# chip = "nrf52833_xxAA" # para micro:bit V2, descomente esta linha
# chip = "nrf51822_xxAA" # para micro:bit V1, descomente esta linha
```

<!-- If you are working with the micro:bit v2 board uncomment the first, for the v1
uncomment the second line. -->

Se você estiver trabalhando com a placa micro:bit v2, descomente a primeira
linha, para a v1 descomente a segunda.

<!-- Next run one of these commands: -->

Em seguida, execute um destes comandos:

```
$ # certifique-se de estar no diretório src/03-setup do código-fonte do livro
$ # Se você estiver trabalhando com micro:bit v2
$ rustup target add thumbv7em-none-eabihf
$ cargo embed --target thumbv7em-none-eabihf

$ # Se você estiver trabalhando com micro:bit v1
$ rustup target add thumbv6m-none-eabi
$ cargo embed --target thumbv6m-none-eabi
```

<!-- If everything works correctly cargo-embed should first compile the small example
program in this directory, then flash it and finally open a nice text based user
interface that prints Hello World. -->

Se tudo funcionar corretamente, o cargo-embed deverá primeiro compilar o pequeno
programa de exemplo neste diretório, então gravá-lo e, finalmente, abrirá uma
interface de usuário baseada em texto que imprime "Hello World".

<!-- (If it does not, check out [general troubleshooting] instructions.) -->

(Se não funcionar, consulte as instruções em [Solucionando problemas gerais].)

[Solucionando problemas gerais]: ../appendix/1-general-troubleshooting/index.html

<!-- This output is coming from the small Rust program you just flashed on to your
micro:bit. Everything is working properly and you can continue with the next
chapters! -->

Essa saída está vindo do pequeno programa Rust que você acabou de gravar no seu
micro:bit. Tudo está funcionando corretamente e você pode continuar com os
próximos capítulos!
