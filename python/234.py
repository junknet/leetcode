# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:

    def isPalindrome(self, head: ListNode) -> bool:
        self.left = head

        def traverse(right: ListNode):
            if right == None:
                return True
            res = traverse(right.next) & (self.left.val == right.val)
            self.left = self.left.next
            return res
        return traverse(head)
