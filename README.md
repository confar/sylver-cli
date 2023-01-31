<p align="center">
    <a href="https://sylver.dev"><img src="https://raw.githubusercontent.com/sylver-dev/sylver-cli/master/logo.png" height="100" alt="Sylver logo"/></a>
</p>
<h3 align="center">
  Multi-language programmable linter
</h3>

<div align="center" style="margin-top: 2rem">
    <a href="https://blog.sylver.dev">
        <img src="https://img.shields.io/badge/Hashnode-2962FF?style=for-the-badge&logo=hashnode&logoColor=white" alt="Hashnode"/>
    </a>
    <a href="https://discord.gg/PaVTgTSSxu">
        <img src="https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white" alt="Discord"/>
    </a>
    <a href="https://twitter.com/Geoffrey198">
        <img src="https://img.shields.io/badge/Twitter-%231DA1F2.svg?style=for-the-badge&logo=Twitter&logoColor=white" alt="Twitter"/>
    </a>
    <a href="https://codecov.io/gh/sylver-dev/sylver-cli">
        <img src="https://codecov.io/gh/sylver-dev/sylver-cli/branch/main/graph/badge.svg?token=GEJ13H3DSG" alt="codecov"/>
    </a>
</div>

# Installation

## Binary releases

```
curl -s https://sylver.dev/install.sh | bash
```

## From source
Sylver is written in Rust, so you need to have the Rust toolchain installed. You can install it from [rustup](https://rustup.rs/). 
To build sylver:
```
git clone https://github.com/sylver-dev/sylver-cli.git
cd sylver-cli 
cargo build --release
```

# Running your first analysis

The following command will automatically detect the language(s) of your project and install the corresponding rulesets
from the registry:
```bash
sylver init
```

You can then run the analysis:
```bash
sylver check
```

The ruleset definitions available at [https://github.com/sylver-dev/rulesets](https://github.com/sylver-dev/rulesets).

# Writing your own rulesets

* Tutorial for a Python: [blog.sylver.dev](https://blog.sylver.dev/build-a-custom-python-linter-in-5-minutes)
* Tutorial for a Javascript: [blog.sylver.dev](https://blog.sylver.dev/build-a-custom-javascript-linter-in-5-minutes)
* Tutorial for a Go linter: [blog.sylver.dev](https://blog.sylver.dev/build-a-custom-go-linter-in-5-minutes)
* Tutorial for a JSON: [blog.sylver.dev](https://blog.sylver.dev/building-a-json-validator-with-sylver-part13-writing-a-json-parser-in-49-lines-of-code)
