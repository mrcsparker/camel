# NIF for Elixir.Camel

https://github.com/LLukas22/llm-rs-python/blob/main/src/lib.rs - using as a starting point.

## To build the NIF module:

- Your NIF will now build along with your project.

## To load the NIF:

```elixir
defmodule Camel do
  use Rustler, otp_app: :ellem, crate: "camel"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
```

## Examples

[This](https://github.com/rusterlium/NifIo) is a complete example of a NIF written in Rust.
