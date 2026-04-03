// EXERCISE 1: Fix the remove() method of a singly linked list.
//
// remove(val) should delete ALL nodes whose value equals val.
// The bug: after deleting a node, it advances `curr` past the new next node,
// so consecutive duplicates are silently skipped.
//
// Hint: only advance curr when you did NOT delete — after a deletion, the
// new curr->next is already the candidate to check next.
//
// Build & run:
//   cmake --build build --target ex01_linked_list && ./build/module_01_data_structures/ex01_linked_list

#include <cstdio>
#include <vector>
#include <initializer_list>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct Node {
    int val;
    Node* next;
    Node(int v, Node* n = nullptr) : val(v), next(n) {}
};

class List {
public:
    Node* head = nullptr;

    void push_back(int v) {
        if (!head) { head = new Node(v); return; }
        Node* c = head;
        while (c->next) c = c->next;
        c->next = new Node(v);
    }

    // Remove all nodes whose value equals val.
    void remove(int val) {
        while (head && head->val == val) {
            Node* tmp = head; head = head->next; delete tmp;
        }
        Node* curr = head;
        while (curr && curr->next) {
            if (curr->next->val == val) {
                Node* tmp = curr->next;
                curr->next = tmp->next;
                delete tmp;
                curr = curr->next;
            } else {
                curr = curr->next;
            }
        }
    }

    std::vector<int> to_vec() const {
        std::vector<int> v;
        for (Node* n = head; n; n = n->next) v.push_back(n->val);
        return v;
    }

    ~List() { while (head) { Node* n = head->next; delete head; head = n; } }
};

int main() {
    // Non-consecutive occurrences
    {
        List l;
        for (int x : {1,2,3,2,4}) l.push_back(x);
        l.remove(2);
        CHECK(l.to_vec() == (std::vector<int>{1,3,4}));
    }
    // Consecutive duplicates — the tricky case
    {
        List l;
        for (int x : {1,2,2,2,3}) l.push_back(x);
        l.remove(2);
        CHECK(l.to_vec() == (std::vector<int>{1,3}));
    }
    // Target at head
    {
        List l;
        for (int x : {5,5,1,2}) l.push_back(x);
        l.remove(5);
        CHECK(l.to_vec() == (std::vector<int>{1,2}));
    }
    // Remove every element
    {
        List l;
        for (int x : {7,7,7}) l.push_back(x);
        l.remove(7);
        CHECK(l.to_vec() == (std::vector<int>{}));
    }
    // Target absent
    {
        List l;
        for (int x : {1,2,3}) l.push_back(x);
        l.remove(9);
        CHECK(l.to_vec() == (std::vector<int>{1,2,3}));
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
