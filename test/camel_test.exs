defmodule CamelTest do
  use ExUnit.Case
  doctest Camel

  test "greets the world" do
    assert Camel.hello() == :world
  end
end
