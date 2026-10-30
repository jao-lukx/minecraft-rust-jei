# Validação — 30/10/2026

- Rust 1.99.1: `cargo test`, `cargo clippy --all-targets -- -D warnings` e formatação dos módulos.
- Cliente real, base offline publicada: `cargo check --locked --no-default-features -p horizon-client --bin horizon-client`, com os quatro mods instalados juntos.
- Instaladores: instalação conjunta, repetição sem mudanças, remoção e reinstalação independente, recusa de fonte incompatível. Nenhum recurso do Minecraft foi usado.
- O cliente não foi iniciado: interface, controles interativos, multiplayer e FPS **não foram testados**. Não há benchmark que comprove ganho de desempenho.
- `cargo check` verifica tipos e integração; não gera um executável distribuível. O usuário precisa compilar conforme o README. Os ZIPs publicados são pacotes de código-fonte.

Para repetir o teste do instalador sobre uma cópia limpa do código offline:

```sh
python3 tools/test_install.py /caminho/horizon-offline-profiles
```
