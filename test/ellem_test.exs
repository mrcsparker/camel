defmodule EllemTest do
  use ExUnit.Case
  doctest Ellem

  test "greets the world" do
    assert Ellem.hello() == :world
  end
end
