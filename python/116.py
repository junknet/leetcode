
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next


class Solution:
    def connect(self, root: 'Node') -> 'Node':
        def connectTwo(node1: 'Node', node2: 'Node') -> 'Node':
            if node1 == None or node2 == None:
                return
            node1.next = node2
            connectTwo(node1.right, node2.left)
            connectTwo(node1.left, node1.right)
            connectTwo(node2.left, node2.right)

        if root == None:
            return None
        connectTwo(root.left, root.right)
        return root
