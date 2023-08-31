defmodule Ellem do
  @moduledoc """
  Documentation for `Ellem`.
  """

  use Rustler,
    otp_app: :ellem,
    crate: :ellem

  def add(_, _), do: :erlang.nif_error(:nif_not_loaded)

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
