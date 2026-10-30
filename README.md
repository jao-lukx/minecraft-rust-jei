# JEI Rust

Lista de itens, busca e receitas/usos com R e U.

Implementação independente para [Minecraft Rust Offline / Horizon](https://github.com/jao-lukx/minecraft-rust-offline), Minecraft 26.3. Não é o mod Java original nem é afiliado aos seus autores.

## Instalar na versão offline

Este é um **mod de código-fonte**, não um JAR ou plugin carregado pelo executável pronto. Requer recompilar o cliente. O instalador aplica alterações pequenas no código extraído do ZIP `horizon-offline-profiles-source.zip` da distribuição offline. Não baixa outro Horizon e preserva as alterações dos perfis offline.

1. Baixe e extraia o código-fonte da distribuição offline. Instale Rust 1.99.1 ou mais recente e as dependências Linux descritas no guia da distribuição.
2. Clone este repositório e aplique o mod:

```sh
python3 tools/install.py /caminho/horizon-offline-profiles
```

3. Para combinar os quatro mods, execute o instalador de cada repositório sobre a mesma pasta **antes** de compilar. Eles usam IDs e módulos distintos.
4. Na raiz do código do Horizon:

```sh
cargo build --locked --profile fast -p horizon-client --bin horizon-client
./target/fast/horizon-client --offline --name Builder_1 --resources /caminho/recursos/26.3
```

Use os caminhos dos seus próprios recursos preparados e o seu perfil. Não compartilhe recursos, contas ou mundos. Preserve `HORIZON_HOME` e `HORIZON_LAUNCHER_HOME` que usa hoje. Nenhum mundo é aberto pelo instalador.

O mod aparece em **Opções → Mods** e pode ser desligado ali. Na primeira compilação ele está ligado. Preferências gráficas e de controles originais não são regravadas pelo mod.

## Remover

```sh
python3 tools/install.py /caminho/horizon-offline-profiles --remove
```

Recompile o cliente. A remoção preserva os demais mods. O instalador recusa âncoras incompatíveis e arquivos do mod alterados localmente. Não rode `cargo fmt` no código integrado antes de remover: a remoção exige as âncoras exatas. Guarde uma cópia limpa do código original.

## Desenvolvimento

```sh
cargo test
cargo clippy --all-targets -- -D warnings
python3 tools/package.py
```

`core.rs` é a lógica Rust testável; `runtime.rs` integra com o jogo. `patches.json` descreve cada alteração. Python é usado apenas como ferramenta de desenvolvimento. O pacote `target/*-source.zip` contém fontes e instalador, não um `.hnmod`; não o coloque na pasta de mods do launcher.

Base: Horizon `e4472a7` com perfis offline `34ead8e`. Outras revisões não são garantidas. Veja [VALIDATION.md](VALIDATION.md) para os resultados e limites dos testes.

## Controles e limites

O painel aparece ao lado do inventário e dos contêineres. Clique esquerdo mostra receitas; direito mostra usos. **R** e **U** funcionam sobre itens. Busque por nome/ID, `@namespace` ou `#tag`, combinando termos. A seta da receita preenche a grade apenas para receitas conhecidas, respeitando as regras do servidor; Shift tenta preencher o máximo. Há páginas e botão de voltar.

Inclui crafting, fornos, defumação, fogueira, corte de pedra, ferraria e poções, conforme os dados expostos pelo Horizon. Limites: 8192 itens e 256 receitas por consulta; em servidores, a tabela contém somente os dados que o cliente conhece. Não inclui integração com plugins Java nem criação de itens em sobrevivência.

## Outros mods desta coleção

- [minecraft-rust-mouse-tweaks](https://github.com/jao-lukx/minecraft-rust-mouse-tweaks)
- [minecraft-rust-sodium](https://github.com/jao-lukx/minecraft-rust-sodium)
- [minecraft-rust-zoom](https://github.com/jao-lukx/minecraft-rust-zoom)

## Download dos fontes

[Baixar pacote 0.1.0](https://raw.githubusercontent.com/jao-lukx/minecraft-rust-jei/main/downloads/rust_jei-0.1.0-source.zip). Confira o SHA-256 em `downloads/SHA256SUMS`. Exige recompilar o cliente, conforme as instruções acima.
