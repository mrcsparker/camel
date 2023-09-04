defmodule Ellem do
  @moduledoc """
  Documentation for `Ellem`.
  """

  use Rustler,
    otp_app: :ellem,
    crate: :ellem

  def get_accelerator(), do: :erlang.nif_error(:nif_not_loaded)
  def simple_chat(_, _), do: :erlang.nif_error(:nif_not_loaded)

  def camel(_), do: :erlang.nif_error(:nif_not_loaded)
  def generate(_, _), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Hello world.

  ## Examples

      iex> Ellem.hello()
      :world

  """
  def hello do
    :world
  end
end
