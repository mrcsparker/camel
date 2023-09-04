defmodule Camel do
  @moduledoc """
  Documentation for `Camel`.
  """

  use Rustler,
    otp_app: :camel,
    crate: :camel

  def get_accelerator(), do: :erlang.nif_error(:nif_not_loaded)
  def simple_chat(_, _), do: :erlang.nif_error(:nif_not_loaded)

  def new(_), do: :erlang.nif_error(:nif_not_loaded)
  def generate(_, _), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Hello world.

  ## Examples

      iex> Camel.hello()
      :world

  """
  def hello do
    :world
  end
end
