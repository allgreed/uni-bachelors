// Autor: Olgierd Kasprowicz
// wariant 18

#pragma once
// *******************************
// Warning: this library may throw
// *******************************

#include "list.hpp"
#include <string>

template <typename T>
class MyList : public List<T>
{
    // inheriting constructors
    using List<T>::List;

    public:
        friend void pisana_funkcja(MyList<std::string> & lista_napisow, int pozycja_pe);
};

// For truly type agnostic templates
#include "main.cpp"
