// Autor: Olgierd Kasprowicz
// wariant 18

#pragma once
// *******************************
// Warning: this library may throw
// *******************************

template <typename T>
class List
{
    protected:
        template<typename _>
        struct ListNode
        {
            T value;
            ListNode* next;
            ListNode* previous;

            ListNode(T value, ListNode* previous, ListNode* next)
            : value(value), next(next), previous(previous) {}

            ListNode()
            : next(this), previous(this) {}
        };

    protected:
        ListNode<T>* sentinel;

    public:
        List();

        ~List();
        List(const List<T>& list);
        List<T>& operator=(const List<T>& list);
        List(List<T>&& list);
        List<T>& operator=(List<T>&& list);

        // Basic operations
        void insert(int index, T value);
        T at(int index) const;
        T erase(int index);
        int size() const;

        bool operator==(const List<T>& compared);
        bool operator!=(const List<T>& compared);

    protected:
        ListNode<T>* _node_at(int index) const;
};
