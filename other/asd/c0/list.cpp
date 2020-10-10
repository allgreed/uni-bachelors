#include <stdexcept>

#pragma region Constructors, destructor, assignemt operators

template <typename T>
List<T>::List()
{
    this->sentinel = new ListNode<T>;
}

template <typename T>
List<T>::~List()
{
    ListNode<T> * i = this->sentinel->next;

    while(i != this->sentinel)
    {
        i = i->next;
        delete i->previous;
    }

    delete this->sentinel;
}

template <typename T>
List<T>& List<T>::operator=(const List<T> 
    &list)
{
    if (this != &list)
    {
        ListNode<T> * this_sentinel = this->sentinel;
        ListNode<T> * list_sentinel = list.sentinel;
        ListNode<T> * i = this_sentinel->next;
        ListNode<T> * j = list_sentinel->next;

        while(i != this_sentinel and j != list_sentinel)
        {
            i->value = j->value;
            i = i->next;
            j = j->next;

        }

        if ((not (this_sentinel == i)) and (list_sentinel == j))
        {
            i->previous->next = this_sentinel;

            while(i != this_sentinel)
            {
                i = i->next;
                delete i->previous;
            }
        }
        else if ((this_sentinel == i) and (not (list_sentinel == j)))
        {
            while(j != list_sentinel)
            {
                i->next = new ListNode<T>(j->value, i, this_sentinel);
                this_sentinel->previous = i->next;
                i = i->next;
                j = j->next;
            }
        }
    }

    return *this;
}

template <typename T>
List<T>::List(const List<T> &list) : List()
{
    *this = list;
}


template <typename T>
List<T>& List<T>::operator=(List<T>&& list)
{
    std::swap(this->sentinel, list.sentinel);

    return *this;
}

template <typename T>
List<T>::List(List<T>&& list) : List()
{
    *this = std::move(list);
}

#pragma endregion

#pragma region Private
template <typename T>
List<T>::ListNode<T>* List<T>::_node_at(int index) const
{
    ListNode<T> * i = this->sentinel;
    int count = -1;

    while(count < index)
    {
        i = i->next;
        ++count;

        if (i == this->sentinel)
            throw std::out_of_range("List range exceeded");
    }

    return i;
}

#pragma endregion

#pragma region Basic operations

template <typename T>
void List<T>::insert(int index, T value)
{
    if (index < 0)
        throw std::invalid_argument("Received negative index");

    ListNode<T> * insertion_node = this->_node_at(index - 1);

    ListNode<T> * new_node = new ListNode<T>(value, insertion_node, insertion_node->next);
    insertion_node->next->previous = new_node; 
    insertion_node->next = new_node;
}

template <typename T>
T List<T>::at(int index) const
{
    if (index < 0)
        throw std::invalid_argument("Received negative index");

    return this->_node_at(index)->value;
}

template <typename T>
int List<T>::size() const
{
    int count = 0;

    for (ListNode<T>* i = this->sentinel->next; i != this->sentinel; i = i->next)
    {
        count++;
    }

    return count;
}

template <typename T>
T List<T>::erase(int index)
{
    if (index < 0)
        throw std::invalid_argument("Received negative index");

    ListNode<T>* deletion_node = this->_node_at(index);

    deletion_node->previous->next = deletion_node->next;
    deletion_node->next->previous = deletion_node->previous;

    T retval = deletion_node->value;
    delete deletion_node;

    return retval;
}

template <typename T>
bool List<T>::operator==(const List<T>& other)
{
    ListNode<T> * this_sentinel = this->sentinel;
    ListNode<T> * other_sentinel = other.sentinel;
    ListNode<T> * i = this_sentinel->next;
    ListNode<T> * j = other_sentinel->next;

    while(i != this_sentinel and j != other_sentinel)
    {
        if (i->value != j->value)
            return false;

        i = i->next;
        j = j->next;
    }

    if ((this_sentinel == i) and (other_sentinel == j))
        return true;

    return false;
}

template <typename T>
bool List<T>::operator!=(const List<T>& other)
{
    return not (*this == other);
}
#pragma endregion
