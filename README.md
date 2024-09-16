# Truth Table Generator

> Visit [GitHub Pages](https://starcatmeow.github.io/truth-table-gen/) to use the tool.

[![Build and Deploy](https://github.com/starcatmeow/truth-table-gen/actions/workflows/build-deploy.yaml/badge.svg)](https://github.com/starcatmeow/truth-table-gen/actions/workflows/build-deploy.yaml)

This is a simple truth table generator that can generate truth tables for propositional logic formulas. Supported syntax is similar to [George's](https://student.cs.uwaterloo.ca/~se212/george/george-docs-1/prop.html).

## Implementation

We use [pest](https://github.com/pest-parser/pest) for parsing the input proposition. The PEG is defined in [src/prop.pest](src/prop.pest). And we transform the parse result into an AST (PropAstNode in [src/prop.rs](src/prop.rs)). Then we can generate the truth table by enumerating all boolean evaluations and evaluating all components on the AST.

Then we expose this ability through a function `generate_truth_table` in [src/lib.rs](src/lib.rs), and compile the whole thing into a WebAssembly module using [wasm-pack](https://github.com/rustwasm/wasm-pack). The generated module is then used in our simple frontend to generate the truth table.

## Developers

[@starcatmeow](https://github.com/starcatmeow)
[@encodeous](https://github.com/encodeous)
