#!/usr/bin/env python3
from typing import List


def permutations(my_list: List[int]) -> List[List[int]]:
    result = []
    if len(my_list) > 1:
        for mvar in permutations(my_list[1:]):
            for i in range(len(mvar) + 1):
                result.append(mvar[:i] + [my_list[0]] + mvar[i:])
    else:
        result.append(my_list)
    return result


if __name__ == "__main__":
    n = 4
    list_of_lists_perm = permutations(list(range(1, n + 1)))

    print(len(list_of_lists_perm))
    for a_list in list_of_lists_perm:
        print(" ".join(map(str, a_list)))
