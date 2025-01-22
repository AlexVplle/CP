// @leet start

// #include <cstddef>
// struct ListNode {
//     int val;
//     ListNode *next;
//     ListNode(int x) : val(x), next(NULL) {}
// };

class Solution {
public:
    bool hasCycle(ListNode *head) {
        ListNode *slow = head;
		if (slow == nullptr) {
			return false;
		}
		ListNode *fast = slow->next;
		while (fast != nullptr && fast->next != nullptr) {
			if (slow == fast) {
				return true;
			}
			slow = slow->next;
			fast = fast->next->next;
		}
		return false;
    }
};
// @leet end
