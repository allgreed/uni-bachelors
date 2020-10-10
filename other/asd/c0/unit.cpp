// Autor: Olgierd Kasprowicz
// wariant 18

#include "main.hpp"
#include "unit_list.cpp"

TEST(pisana_funkcja, basic_case_0)
{
    MyList<std::string> list;

    list.insert(0, "a123");
    list.insert(1, "b123");
    list.insert(2, "c123");
    list.insert(3, "d123");
    list.insert(4, "e123");

    pisana_funkcja(list, 2);

    EXPECT_EQ(list.at(0), "a123");
    EXPECT_EQ(list.at(1), "b123");
    EXPECT_EQ(list.at(2), "c123");
    EXPECT_EQ(list.size(), 3);
}

TEST(pisana_funkcja, basic_case_1)
{
    MyList<std::string> list;

    list.insert(0, "zielona");
    list.insert(1, "jest");
    list.insert(2, "moja");
    list.insert(3, "papryka");

    pisana_funkcja(list, 1);

    EXPECT_EQ(list.at(0), "zielona");
    EXPECT_EQ(list.at(1), "jest");
    EXPECT_EQ(list.size(), 2);
}

TEST(pisana_funkcja, list_shorter)
{
    MyList<std::string> list;

    list.insert(0, "f 456");
    list.insert(1, "g 678");

    pisana_funkcja(list, 5);

    EXPECT_EQ(list.at(0), "f 456");
    EXPECT_EQ(list.at(1), "g 678");
    EXPECT_EQ(list.size(), 2);
}

TEST(pisana_funkcja, empty_list)
{
    MyList<std::string> list;

    pisana_funkcja(list, 5);

    EXPECT_EQ(list.size(), 0);
}

TEST(pisana_funkcja, empty_list_zero)
{
    MyList<std::string> list;

    pisana_funkcja(list, 0);

    EXPECT_EQ(list.size(), 0);
}

TEST(pisana_funkcja, zero)
{
    MyList<std::string> list;

    list.insert(0, "to");
    list.insert(0, "jest");
    list.insert(0, "tylko");
    list.insert(0, "test");
    list.insert(0, "to jest istotne");

    pisana_funkcja(list, 0);

    EXPECT_EQ(list.at(0), "to jest istotne");
    EXPECT_EQ(list.size(), 1);
}

TEST(pisana_funkcja, negative_argument)
{
    MyList<std::string> list;

    ASSERT_THROW(pisana_funkcja(list, -8);, std::invalid_argument);
}

