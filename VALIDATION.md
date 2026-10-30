# Validação 0.2.0

- Os cinco mods do conjunto local foram aplicados por horizon-mixins 0.6.0 e o cliente real foi compilado com `cargo build --locked --profile fast -p horizon-client --bin horizon-client` (Linux x86_64, áudio e controles ativados).
- O launcher atualizado foi compilado e sua CLI executada. 3 testes de descoberta/validação de arquivos e 13 testes do pipeline nativo passaram.
- Um jogo-fixture Rust passou pelo pipeline completo do launcher: compilação de dois mods independentes, reabertura sem compilador, remoção, atualização de bytes, reutilização ao recolocar e retorno à base com pasta vazia. O marcador de mundo do teste foi preservado.
- O conjunto real pré-compilado foi instalado pelo launcher, conferido por SHA-256 e reutilizado sem baixar Rust. A CLI do cliente instalado executou.
- Busca, orçamento e zoom têm testes Rust de lógica. Zoom e Full Bright do conjunto local tiveram controles exercitados em host de teste.
- **Não houve partida visual com recursos do Minecraft, teste multiplayer ou medição de FPS.** Compilação e testes de integração não substituem uma partida real.

Os arquivos `.hnmod` são pacotes de código nativo, compilados em conjunto pelo launcher. Só o conjunto para o qual um executável foi fornecido pode reutilizar esse executável pré-compilado.
