class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        complements = {}

        for current_idx, num in enumerate(nums):
            stored_idx = complements.get(num)

            if stored_idx is not None:
                return [stored_idx, current_idx]

            complement = target - num
            complements[complement] = current_idx


if __name__ == "__main__":
    nums = [2, 7, 11, 15]
    target = 9

    solution = Solution().twoSum(nums, target)
    print(solution)
