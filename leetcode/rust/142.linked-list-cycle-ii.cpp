// @leet start
// #include <cstddef>
// struct ListNode {
//     int val;
//     ListNode *next;
//     ListNode(int x) : val(x), next(NULL) {}
// };

class Solution {
public:
	ListNode *intersection(ListNode *head) {
        ListNode *slow = head;
        ListNode *fast = head;
		while (fast != nullptr && fast->next != nullptr) {
			slow = slow->next;
			fast = fast->next->next;
			if (slow == fast) {
				return slow;
			}
		}
		return nullptr;
	}

    ListNode *detectCycle(ListNode *head) {
		ListNode *intersect = intersection(head);
		if (intersect == nullptr) {
			return nullptr;
		}
		ListNode *start = head;
		while (start != intersect) {
			start = start->next;
			intersect = intersect->next;
		}
		return start;
    }
};
// @leet end
