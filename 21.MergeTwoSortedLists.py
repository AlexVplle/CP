class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def extractValue(self, node : ListNode) :
        listValue = []
        while node :
            listValue.append(node.val)
            node = node.next
        return listValue
    
    def recreateListNode(self, arrayValue) :
        if not arrayValue : return None
        return ListNode(arrayValue[0], self.recreateListNode(arrayValue[1:]))
    
    def mergeTwoLists(self, list1: ListNode, list2: ListNode) -> ListNode:
        return self.recreateListNode(sorted(self.extractValue(list1) + self.extractValue(list2)))

# leetcode.com/problems/merge-two-sorted-lists/