class ListNode:
    def __init__(self, val=0, next=None):
        #         self.val = val
        self.next = next


class Solution:
    def reverseKGroup(self, head: ListNode, k: int) -> ListNode:
        tmp = head
        for _ in range(k):
            if tmp == None:
                return head
            tmp = tmp.next
        curr, pre = head, None
        for _ in range(k):
            # pre, pre.next, curr = curr, pre, curr.next
            next = curr.next
            curr.next = pre
            pre = curr
            curr = next

        head.next = self.reverseKGroup(curr, k)
        return pre
