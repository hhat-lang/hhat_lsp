# H-hat LSP


H-hat LSP repository is part of the [H-hat ecosystem](https://github.com/hhat-lang).


This repository is the official place for H-hat's language server protocol (LSP) implementations. It iams to contain the server(s) and various clients for different IDE/code editors that support LSP, as well as future dialects. Currently, it supports only the Heather dialect.


## How to install


This section is under development. You can reach out in the [Discussions page](https://github.com/hhat-lang/hhat_lsp/discussions) or open an issue.



## LSP Implementation


The Heather dialect is the current existing implementation of H-hat rule system, and is the one being carried out for the LSP features. Unless explicitly defined otherwise, everything contained in this repository is with the regards of the Heather dialect LSP implementation.


### Current (in progress) Implementations


- TypeScript version for LSP server and client by @Tamicktom to work with VSCode
- Rust version for LSP server by @Doomsk to work with any IDE/code editor


### Future Implementations


- Kotlin version for LSP client to work with JetBrains' IDEs
- Lua version for LSP client to work with NeoVim


## H-hat Project Template


H-hat has a specific way to handle different parts of the code. There are sections related to the code base (`<project>/src/`) and also to the documentation (`<project>/docs/`).

There are three kinds of H-hat files that you may find according to the roles: functions, types and constants definition.


The rules are: (1) if a file is inside the types folder (`<project>/src/hat_types/`), it **must** be a types definition; (2) if a file is elsewhere inside the code base folder (`/src/`) with the name `consts.hat`, it is a constants definition. Everything else, including `main.hat`, is a functions definition.


## License


This repository uses MIT license for its code.


## Generative AI/LLM Disclaimer


The authors are responsible for the code generated, to truthfully report any usage of generative AI/LLM (see []()). Also, check the [Contributing page](CONTRIBUTING.md).


## How to Contribute


Please check [CONTRIBUTING.md](CONTRIBUTING.md).

