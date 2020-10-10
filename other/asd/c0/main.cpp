// Autor: Olgierd Kasprowicz
// wariant 18

#include "list.cpp"

void pisana_funkcja(MyList<std::string> & lista_napisow, int pozycja_pe)
{
    if (pozycja_pe < 0)
        throw std::invalid_argument("Received negative index");

    MyList<std::string>::ListNode<std::string> * i;

    try
    {
        i = lista_napisow._node_at(pozycja_pe + 1);
    }
    catch(std::out_of_range)
    {
        return;
    }

    i->previous->next = lista_napisow.sentinel;
    lista_napisow.sentinel->previous = i->previous;
    
    MyList<std::string>::ListNode<std::string> * previous = i;
    while(i->next != lista_napisow.sentinel)
    {
        i = i->next;
        delete previous;
        previous = i;
    }
    delete i;
}
