# Ellem

LLMs in Elixir. Being worked on.

Ellem.accelerator

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `ellem` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:ellem, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/ellem>.

## Camel API

This is a test for llama models. It is being developed as a prototype

```elixir
Ellem.camel("./open_llama_3b-f16.bin") |> Ellem.generate("Today is the best day to")
```
