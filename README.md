# Camel

Prototype for LLM in Elixir.

Ellem.accelerator

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `camel` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:camel, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/ellem>.

## API

This is a test for llama models. It is being developed as a prototype

```elixir
Camel.new("./open_llama_3b-f16.bin") |> Camel.generate("Today is the best day to")
```
