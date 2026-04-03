// EXERCISE 5: Implement get() and put() for an LRU (Least Recently Used) cache.
//
// The cache holds at most `capacity` items. When it would exceed capacity on a put(),
// evict the least-recently-used entry. Both get() and put() count as a "use".
//
// Scaffold provided:
//   - Node: doubly-linked list node (key, val, prev, next)
//   - Dummy head/tail sentinels (head = MRU side, tail = LRU side)
//   - unlink(n):        remove a node from its current list position
//   - insert_at_front(n): place a node just after the head sentinel (MRU position)
//
// You implement:
//   get(key)  — look up in map; if found, move to front and return val; else return -1
//   put(key, val) — if key exists, update val and move to front
//                   if new, create node and insert at front
//                   if over capacity, evict the node just before tail (LRU)
//
// Build & run:
//   cmake --build build --target ex05_lru_cache && ./build/module_01_data_structures/ex05_lru_cache

#include <cstdio>
#include <unordered_map>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct Node {
    int key, val;
    Node *prev = nullptr, *next = nullptr;
    Node(int k, int v) : key(k), val(v) {}
};

class LRUCache {
public:
    int capacity;
    std::unordered_map<int, Node*> map;
    Node* head;  // dummy sentinel — MRU side
    Node* tail;  // dummy sentinel — LRU side

    explicit LRUCache(int cap) : capacity(cap) {
        head = new Node(0, 0);
        tail = new Node(0, 0);
        head->next = tail;
        tail->prev = head;
    }

    ~LRUCache() {
        Node* curr = head;
        while (curr) { Node* n = curr->next; delete curr; curr = n; }
    }

    // Detach node from its current position in the list.
    void unlink(Node* n) {
        n->prev->next = n->next;
        n->next->prev = n->prev;
    }

    // Place node immediately after the head sentinel (most-recently-used position).
    void insert_at_front(Node* n) {
        n->next = head->next;
        n->prev = head;
        head->next->prev = n;
        head->next = n;
    }

    // TODO: Return the value for key (and mark it most-recently-used), or -1 if absent.
    int get(int key) {
        // your code here
        return -1;
    }

    // TODO: Insert or update key->val.
    //       New entries go to the MRU position; evict the LRU entry when over capacity.
    void put(int key, int val) {
        // your code here
    }
};

int main() {
    // Classic LRU sequence
    {
        LRUCache c(2);
        c.put(1, 1);
        c.put(2, 2);
        CHECK(c.get(1) == 1);   // access 1 → 1 becomes MRU, 2 becomes LRU
        c.put(3, 3);            // capacity full → evict 2 (LRU)
        CHECK(c.get(2) == -1);  // 2 was evicted
        CHECK(c.get(3) == 3);
        c.put(4, 4);            // evict 1 (LRU now: get(1) was earlier than get(3))
        CHECK(c.get(1) == -1);
        CHECK(c.get(3) == 3);
        CHECK(c.get(4) == 4);
    }

    // put() on existing key updates value and refreshes recency
    {
        LRUCache c(2);
        c.put(1, 1);
        c.put(2, 2);
        c.put(1, 10);           // update key 1 → 1 is now MRU, 2 is LRU
        c.put(3, 3);            // evict 2
        CHECK(c.get(1) == 10);
        CHECK(c.get(2) == -1);
        CHECK(c.get(3) == 3);
    }

    // Three-item cache
    {
        LRUCache c(3);
        c.put(1, 1); c.put(2, 2); c.put(3, 3);
        c.get(1);               // access order (MRU→LRU): 1, 3, 2
        c.put(4, 4);            // evict 2
        CHECK(c.get(2) == -1);
        CHECK(c.get(1) == 1);
        CHECK(c.get(3) == 3);
        CHECK(c.get(4) == 4);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
