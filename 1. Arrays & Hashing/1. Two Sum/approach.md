# Task
Given an array of integers `nums` and an integer `target`,
return indices of the two numbers such that they add up to target.


# Given Values
Inputs = nums: Vec<i32>, target: i32,
Outputs = Vec<i32>,

Example:
Input: nums = [2,7,11,15], target = 9
Output: [0,1]


# Naive Solution
## Reasoning
Run a nested loop and compare values until it works.

## Complexity
Time - O(n^2):
    A double nested loop

Space - O(1):
    Only a constant number of variables are used

## Pseudocode
```
for i in 0..nums.len():
    for j in (i + 1)..nums.len():
        if nums[i] + nums[j] == target:
            return [i, j]
```


# Ideal Solution
## Reasoning
You need to quickly determine if a number's required complement has already appeared.
A hashmap gives you an O(1) average-case lookup for that question.

For example:
```
target = 9

num = 2, complement = 7
map = {7: 0}

num = 7
7 exists in map => [0, 1]
```

Create a hashmap.
Loop through the list of numbers.
Check if the number is in the map.
    If it is then return the stored index and the current index
    Else insert Key: (Target - num), Value: (index) to the hashmap



Note: We only need fast lookup; we do not need the keys to be ordered.
As such a hashmap is preferable to an ordered map such as a TreeMap.

Note: Check before inserting to the map.
This is done to prevent the current number matching itself in certain cases.

Consider:
```
nums = [3, 4, 2]
target = 6
```
If you inserted first and then checked, 3 could find itself.
Checking first ensures that the map only represents previous elements.

## Complexity
Time - O(n):
    One pass through the array
    O(1) average-case hashmap lookup/insertion.

Space - O(n):
    The hashmap can contain up to n entries

## Pseudocode
```
map = Hashmap<num, index>

for i, num in nums.enumerate():
    res_id = map.get(num)
    if res_id != None:
        return [res_id, i]
    else:
        ans = target - num
        map.insert(ans, i)
    
```
