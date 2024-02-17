<!-- # Debug it -->

# Depurando

<!-- ## How does this even work? -->

## Como isso funciona?

<!-- Before we debug our little program let's take a moment to quickly understand what is actually
happening here. In the previous chapter we already discussed the purpose of the second chip
on the board as well as how it talks to our computer, but how can we actually use it? -->

Antes de depurar nosso pequeno programa, vamos dedicar um momento para entender
rapidamente o que realmente está acontecendo aqui. No capítulo anterior, já
discutimos o propósito do segundo chip na placa e também como ele se comunica
com o nosso computador, mas como podemos realmente usá-lo?

<!-- The little option `default.gdb.enabled = true` in `Embed.toml` made
`cargo-embed` open a so-called "GDB stub" after flashing, this is a server that
our GDB can connect to and send commands like "set a breakpoint at address X"
to. The server can then decide on its own how to handle this command. In the
case of the `cargo-embed` GDB stub it will forward the command to the debugging
probe on the board via USB which then does the job of actually talking to the
MCU for us. -->

A pequena opção `default.gdb.enabled = true` no arquivo `Embed.toml` fez com que
o `cargo-embed` abrisse um chamado "GDB stub" após o flashing, que é um servidor
ao qual nosso GDB pode se conectar e enviar comandos como "definir um breakpoint
no endereço X". O servidor pode então decidir por conta própria como lidar com
esse comando. No caso do `cargo-embed` GDB stub, ele encaminhará o comando para
a sonda de depuração na placa via USB, que por sua vez faz o trabalho de
realmente se comunicar com a MCU para nós.

<!-- ## Let's debug! -->

## Vamos depurar!

<!-- Since `cargo-embed` is blocking our current shell we can simply open a new one
and cd back into our project directory. Once we are there we first have to open
the binary in gdb like this: -->

Como o `cargo-embed` está bloqueando nosso shell atual, podemos simplesmente
abrir um novo e voltar ao diretório do nosso projeto. Uma vez lá, primeiro temos
que abrir o binário no gdb assim:

```shell
# Para micro:bit v2
$ gdb target/thumbv7em-none-eabihf/debug/led-roulette

# Para micro:bit v1
$ gdb target/thumbv6m-none-eabi/debug/led-roulette
```

<!-- > **NOTE**: Depending on which GDB you installed you will have to use a
> different command to launch it, check out [chapter 3] if you forgot which one
> it was. -->

> **NOTA**: Dependendo de qual GDB você instalou, você terá que usar um comando
> diferente para iniciá-lo. Verifique o Capítulo 3 se você esqueceu qual era.

<!-- [chapter 3]: ../03-setup/index.md#tools -->

[Capítulo 3]: ../03-setup/index.md#tools

<!-- > **NOTE**: If you are getting
> `target/thumbv7em-none-eabihf/debug/led-roulette: No such file or directory`
> error, try adding `../../` to the file path, for example:
>
> ```shell
> $ gdb ../../target/thumbv7em-none-eabihf/debug/led-roulette
> ```
>
> This is caused by each example project being in a `workspace` that contains
> the entire book, and workspaces have a single `target` directory. Check out
> [Workspaces chapter in Rust Book] for more. -->

> **NOTA**: Se você estiver recebendo o erro
> `target/thumbv7em-none-eabihf/debug/led-roulette: No such file or directory`,
> tente adicionar `../../` ao caminho do arquivo, por exemplo:
>
> ```shell
> $ gdb ../../target/thumbv7em-none-eabihf/debug/led-roulette
> ```
>
> Isso ocorre porque cada projeto de exemplo está em um `workspace` que contém
> todo o livro, e os workspaces possuem um único diretório de `destino`. Confira
> o [capítulo de Workspaces no Livro de Rust] para mais informações.

<!-- [Workspaces chapter in Rust Book]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html#creating-a-workspace -->

[capítulo de Workspaces no Livro de Rust]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html#creating-a-workspace

<!-- > **NOTE**: If `cargo-embed` prints a lot of warnings here don't worry about it.
> As of now it does not fully implement the GDB protocol and thus might not
> recognize all the commands your GDB is sending to it, as long as it does not
> crash, you are fine. -->

> **NOTA**: Se o `cargo-embed` imprimir muitos avisos aqui, não se preocupe com
> isso. Até o momento, ele não implementa totalmente o protocolo GDB e,
> portanto, pode não reconhecer todos os comandos que seu GDB está enviando para
> ele. Contanto que não ocorra uma falha, está tudo bem.

<!-- Next we will have to connect to the GDB stub. It runs on `localhost:1337` per
default so in order to connect to it run the following: -->

Em seguida, teremos que nos conectar ao GDB stub. Ele é executado em
`localhost:1337` por padrão, portanto, para se conectar a ele, execute o
seguinte comando:

```shell
(gdb) target remote :1337
Remote debugging using :1337
0x00000116 in nrf52833_pac::{{impl}}::fmt (self=0xd472e165, f=0x3c195ff7) at /home/nix/.cargo/registry/src/github.com-1ecc6299db9ec823/nrf52833-pac-0.9.0/src/lib.rs:157
157     #[derive(Copy, Clone, Debug)]
```

<!-- Next what we want to do is get to the main function of our program. We will do
this by first setting a breakpoint there and the continuing program execution
until we hit the breakpoint: -->

A seguir, o que queremos fazer é chegar à função principal do nosso programa.
Faremos isso primeiro configurando um breakpoint lá e continuando a execução do
programa até atingirmos ele:

```
(gdb) break main
Breakpoint 1 at 0x104: file src/05-led-roulette/src/main.rs, line 9.
Note: automatically using hardware breakpoints for read-only addresses.
(gdb) continue
Continuing.

Breakpoint 1, led_roulette::__cortex_m_rt_main_trampoline () at src/05-led-roulette/src/main.rs:9
9       #[entry]
```

<!-- Breakpoints can be used to stop the normal flow of a program. The `continue`
command will let the program run freely _until_ it reaches a breakpoint. In this
case, until it reaches the `main` function because there's a breakpoint there. -->

Breakpoints podem ser usados para interromper o fluxo normal de um programa. O
comando `continue` permitirá que o programa seja executado livremente _até_ que
ele atinja um breakpoint. Nesse caso, ele continuará a execução até chegar à
função `main`, pois há um breakpoint lá.

<!-- Note that GDB output says "Breakpoint 1". Remember that our processor can only
use a limited amount of these breakpoints, so it's a good idea to pay attention
to these messages. If you happen to run out of breakpoints, you can list all the
current ones with `info break` and delete desired ones with
`delete <breakpoint-num>`. -->

Observe que a saída do GDB diz "Breakpoint 1". Lembre-se de que nosso
processador só pode usar uma quantidade limitada desses breakpoints, então é uma
boa ideia prestar atenção nessas mensagens. Se você acabar ficando sem
breakpoints, pode listar todos os atuais com o comando `info break` e excluir os
desejados com `delete <número-do-breakpoint>`.

<!-- For a nicer debugging experience, we'll be using GDB's Text User Interface
(TUI). To enter into that mode, on the GDB shell enter the following command: -->

Para uma experiência de depuração mais agradável, estaremos usando a Interface
de Texto do Usuário (TUI) do GDB. Para entrar neste modo, no shell do GDB,
digite o seguinte comando:

```
(gdb) layout src
```

<!-- > **NOTE**: Apologies Windows users. The GDB shipped with the GNU ARM Embedded
> Toolchain doesn't support this TUI mode `:-(`. -->

> **NOTA**: Pedimos desculpas aos usuários do Windows. O GDB fornecido com a GNU
> ARM Embedded Toolchain não suporta o modo TUI `:(`.

![GDB session](../assets/gdb-layout-src.png "GDB TUI")

<!-- GDB's break command does not only work for function names, it can also break at
certain line numbers. If we wanted to break in line 13 we can simply do: -->

O comando "break" do GDB não funciona apenas para nomes de funções, ele também
pode interromper em determinados números de linha. Se quisermos interromper na
linha 13, podemos simplesmente fazer o seguinte:

```
(gdb) break 13
Breakpoint 2 at 0x110: file src/05-led-roulette/src/main.rs, line 13.
(gdb) continue
Continuing.

Breakpoint 2, led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:13
(gdb)
```

<!-- At any point you can leave the TUI mode using the following command: -->

A qualquer momento, você pode sair do modo TUI usando o seguinte comando:

```
(gdb) tui disable
```

<!-- We are now "on" the `_y = x` statement; that statement hasn't been executed yet.
This means that `x` is initialized but `_y` is not. Let's inspect those
stack/local variables using the `print` command: -->

Agora estamos "na" instrução `_y = x`; essa instrução ainda não foi executada.
Isso significa que `x` está inicializado, mas `_y` não está. Vamos inspecionar
essas variáveis da pilha/locais usando o comando `print`:

```
(gdb) print x
$1 = 42
(gdb) print &x
$2 = (*mut i32) 0x20003fe8
(gdb)
```

<!-- As expected, `x` contains the value `42`. The command `print &x` prints the
address of the variable `x`. The interesting bit here is that GDB output shows
the type of the reference: `i32*`, a pointer to an `i32` value. -->

Conforme esperado, `x` contém o valor `42`. O comando `print &x` imprime o
endereço da variável `x`. A parte interessante aqui é que a saída do GDB mostra
o tipo da referência: `i32*`, um ponteiro para um valor `i32`.

<!-- If we want to continue the program execution line by line we can do that using
the `next` command so let's proceed to the `loop {}` statement: -->

Se quisermos continuar a execução do programa linha por linha, podemos fazer
isso usando o comando `next`, então vamos prosseguir para a instrução `loop {}`:

```
(gdb) next
16          loop {}
```

<!-- And `_y` should now be initialized. -->

E `_y` agora deve estar inicializado.

```
(gdb) print _y
$5 = 42
```

<!-- Instead of printing the local variables one by one, you can also use the
`info locals` command: -->

Em vez de imprimir as variáveis locais uma por uma, você também pode usar o
comando `info locals`:

```
(gdb) info locals
x = 42
_y = 42
(gdb)
```

<!-- If we use `next` again on top of the `loop {}` statement, we'll get stuck
because the program will never pass that statement. Instead, we'll switch to the
disassemble view with the `layout asm` command and advance one instruction at a
time using `stepi`. You can always switch back into Rust source code view later
by issuing the `layout src` command again. -->

Se usarmos `next` novamente em cima da instrução `loop {}`, ficaremos presos,
pois o programa nunca passará por essa instrução. Em vez disso, vamos mudar para
a visualização de desmontagem com o comando `layout asm` e avançar uma instrução
de cada vez usando `stepi`. Você sempre pode voltar para a visualização do
código-fonte em Rust posteriormente emitindo o comando `layout src` novamente.

<!-- > **NOTE**: If you used the `next` or `continue` command by mistake and GDB got
> stuck, you can get unstuck by hitting `Ctrl+C`. -->

> **NOTA**: Se você usou o comando `next` ou `continue` por engano e o GDB ficou
> preso, você pode desbloqueá-lo pressionando `Ctrl+C`.

```
(gdb) layout asm
```

![GDB session](../assets/gdb-layout-asm.png "GDB disassemble")

<!-- If you are not using the TUI mode, you can use the `disassemble /m` command to
disassemble the program around the line you are currently at. -->

Se você não estiver usando o modo TUI, pode usar o comando `disassemble /m` para
desmontar o programa em torno da linha em que você está atualmente.

```
(gdb) disassemble /m
Dump of assembler code for function _ZN12led_roulette18__cortex_m_rt_main17h3e25e3afbec4e196E:
10      fn main() -> ! {
   0x0000010a <+0>:     sub     sp, #8
   0x0000010c <+2>:     movs    r0, #42 ; 0x2a

11          let _y;
12          let x = 42;
   0x0000010e <+4>:     str     r0, [sp, #0]

13          _y = x;
   0x00000110 <+6>:     str     r0, [sp, #4]

14
15          // infinite loop; just so we don't leave this stack frame
16          loop {}
=> 0x00000112 <+8>:     b.n     0x114 <_ZN12led_roulette18__cortex_m_rt_main17h3e25e3afbec4e196E+10>
   0x00000114 <+10>:    b.n     0x114 <_ZN12led_roulette18__cortex_m_rt_main17h3e25e3afbec4e196E+10>

End of assembler dump.
```

<!-- See the fat arrow `=>` on the left side? It shows the instruction the processor
will execute next. -->

Vê a seta gorda `=>` no lado esquerdo? Ela mostra a instrução que o processador
executará em seguida.

<!-- If not inside the TUI mode on each `stepi` command GDB will print the statement
and the line number of the instruction the processor will execute next. -->

Se você não estiver no modo TUI, em cada comando `stepi`, o GDB imprimirá a
declaração e o número da linha da instrução que o processador executará em
seguida.

```
(gdb) stepi
16          loop {}
(gdb) stepi
16          loop {}
```

<!-- One last trick before we move to something more interesting. Enter the following
commands into GDB: -->

Um último truque antes de passarmos para algo mais interessante. Digite os
seguintes comandos no GDB:

```
(gdb) monitor reset
(gdb) c
Continuing.

Breakpoint 1, led_roulette::__cortex_m_rt_main_trampoline () at src/05-led-roulette/src/main.rs:9
9       #[entry]
(gdb)
```

<!-- We are now back at the beginning of `main`! -->

Agora estamos de volta ao início de `main`!

<!-- `monitor reset` will reset the microcontroller and stop it right at the program
entry point. The following `continue` command will let the program run freely
until it reaches the `main` function that has a breakpoint on it. -->

`monitor reset` irá reiniciar o microcontrolador e pará-lo exatamente no ponto
de entrada do programa. O comando `continue` seguinte permitirá que o programa
seja executado livremente até chegar à função `main`, que possui um breakpoint.

<!-- This combo is handy when you, by mistake, skipped over a part of the program
that you were interested in inspecting. You can easily roll back the state of
your program back to its very beginning. -->

Essa combinação é útil quando você, por engano, pulou uma parte do programa que
estava interessado em inspecionar. Você pode facilmente reverter o estado do seu
programa de volta ao seu início.

<!-- > **The fine print**: This `reset` command doesn't clear or touch RAM. That
> memory will retain its values from the previous run. That shouldn't be a
> problem though, unless your program behavior depends on the value of
> _uninitialized_ variables but that's the definition of Undefined Behavior
> (UB). -->

> **Detalhe**: O comando `reset` não limpa ou altera a RAM. Essa memória reterá
> seus valores da execução anterior. Isso não deve ser um problema, a menos que
> o comportamento do seu programa dependa do valor de variáveis _não
> inicializadas_, mas essa é a definição de Comportamento Indefinido (Undefined
> Behavior - UB).

<!-- We are done with this debug session. You can end it with the `quit` command. -->

Concluímos esta sessão de depuração. Você pode encerrá-la com o comando `quit`.

```
(gdb) quit
A debugging session is active.

        Inferior 1 [Remote target] will be detached.

Quit anyway? (y or n) y
Detaching from program: $PWD/target/thumbv7em-none-eabihf/debug/led-roulette, Remote target
Ending remote debugging.
[Inferior 1 (Remote target) detached]
```

<!-- > **NOTE**: If the default GDB CLI is not to your liking check out
> [gdb-dashboard]. It uses Python to turn the default GDB CLI into a dashboard
> that shows registers, the source view, the assembly view and other things. -->

> **NOTA**: Se a CLI padrão do GDB não for do seu agrado, confira o
> [gdb-dashboard]. Ele usa Python para transformar a interface padrão do GDB em
> um painel que mostra os registros, a visualização do código-fonte, a
> visualização em assembly e outras coisas.

[gdb-dashboard]: https://github.com/cyrus-and/gdb-dashboard#gdb-dashboard

<!-- If you want to learn more about what GDB can do, check out the section
[How to use GDB](../appendix/2-how-to-use-gdb/). -->

Se você deseja aprender mais sobre o que o GDB pode fazer, confira a seção
[Como usar o GDB](../appendix/2-how-to-use-gdb/).

<!-- What's next? The high level API I promised. -->

O que vem a seguir? A API de alto nível que eu prometi.
