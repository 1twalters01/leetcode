defmodule Solution do
  @spec two_sum(nums :: [integer], target :: integer) :: [integer]
  def two_sum(nums, target) do
    complements = %{}

    Enum.reduce_while(
      Enum.with_index(nums),
      complements,
      fn {num, current_idx}, complements ->
        case Map.fetch(complements, num) do
          {:ok, stored_idx} -> {:halt, [stored_idx, current_idx]}
          :error -> {:cont, Map.put(complements, target - num, current_idx)}
        end
      end
    )
  end
end


nums = [2, 7, 11, 15]
target = 9

solution = Solution.two_sum(nums, target)
IO.inspect(solution)
