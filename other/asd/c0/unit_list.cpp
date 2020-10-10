#include "main.hpp"
#include "gtest/gtest.h"

TEST(List, Default_parameters)
{
    List<int> list;

    ASSERT_EQ(list.size(), 0);
}

TEST(List, Out_of_bounds_access)
{
    List<int> list;

    ASSERT_THROW(list.at(10), std::out_of_range);
}

TEST(List, Negative_access)
{
    List<int> list;

    ASSERT_THROW(list.at(-2), std::invalid_argument);
}

TEST(List, Negative_insertion_attempt)
{
    List<int> list;

    ASSERT_THROW(list.insert(-2, 0), std::invalid_argument);
}

TEST(List, Out_of_bounds_insertion_attempt)
{
    List<int> list;

    ASSERT_THROW(list.insert(10, 0), std::out_of_range);
}


TEST(List, Get_and_set)
{
    List<int> list;

    list.insert(0, 777);
    EXPECT_EQ(list.at(0), 777);

    list.insert(1, 69);
    EXPECT_EQ(list.at(0), 777);
    EXPECT_EQ(list.at(1), 69);

    list.insert(0, -1000000);
    EXPECT_EQ(list.at(0), -1000000);
    EXPECT_EQ(list.at(1), 777);
    EXPECT_EQ(list.at(2), 69);

    list.insert(2, 0);
    EXPECT_EQ(list.at(0), -1000000);
    EXPECT_EQ(list.at(1), 777);
    EXPECT_EQ(list.at(2), 0);
    EXPECT_EQ(list.at(3), 69);
}

TEST(List, Erase)
{
    List<int> list;
    list.insert(0, 567890);
    list.insert(0, 123456);
    list.insert(0, -123456);

    EXPECT_EQ(list.erase(0), -123456);
    EXPECT_EQ(list.at(0), 123456);
    EXPECT_EQ(list.at(1), 567890);

    EXPECT_EQ(list.erase(1), 567890);
    EXPECT_EQ(list.at(0), 123456);
}

TEST(List, Out_of_bounds_erase)
{
    List<int> list;

    ASSERT_THROW(list.erase(10), std::out_of_range);
}

TEST(List, Negative_erase)
{
    List<int> list;

    ASSERT_THROW(list.erase(-2), std::invalid_argument);
}

TEST(List, Size)
{
    List<int> list;

    EXPECT_EQ(list.size(), 0);

    list.insert(0, 111);
    list.insert(1, 222);

    EXPECT_EQ(list.size(), 2);

    list.erase(0);

    EXPECT_EQ(list.size(), 1);
}

TEST(List, Equality_and_nonequality_operator)
{
    List<int> refence_list;
    List<int> list;

    EXPECT_TRUE(refence_list == list);
    EXPECT_FALSE(refence_list != list);

    list.insert(0, -1);

    EXPECT_TRUE(refence_list != list);
    EXPECT_FALSE(refence_list == list);

    list.erase(0);

    EXPECT_TRUE(refence_list == list);
    EXPECT_FALSE(refence_list != list);

    refence_list.insert(0, 34987384);
    refence_list.insert(1, -1);

    list.insert(0, 34987384);
    list.insert(1, -1);

    EXPECT_TRUE(refence_list == list);
    EXPECT_FALSE(refence_list != list);

    refence_list.insert(list.size(), 5);
    list.insert(list.size(), 0);

    EXPECT_TRUE(refence_list != list);
    EXPECT_FALSE(refence_list == list);
}

TEST(List, Copy_assignment)
{
    List<int> reference_list;
    List<int> list;

    // equal sizes, different values
    reference_list.insert(0, 256);
    reference_list.insert(0, 512);
    list.insert(0, -256);
    list.insert(0, -512);

    list = reference_list;
    EXPECT_TRUE(reference_list == list);

    // reference is smaller
    reference_list.erase(0);
    reference_list.erase(0);

    list = reference_list;
    EXPECT_TRUE(reference_list == list);

    // reference is bigger
    reference_list.insert(0, 256);
    reference_list.insert(0, 512);

    list = reference_list;
    EXPECT_TRUE(reference_list == list);

}

TEST(List, Copy_contructor)
{
    List<int> reference_list;
    List<int> empty_list = reference_list;

    EXPECT_TRUE(reference_list == empty_list);

    reference_list.insert(0, 256);

    List<int> list = reference_list;

    EXPECT_TRUE(reference_list == list);
}

TEST(List, Move_assignment)
{
    List<int> reference_list;
    reference_list.insert(0, 256);
    reference_list.insert(0, 512);
    List<int> disposable_list = reference_list;
    
    List<int> list;
    list.insert(0, 5);

    list = std::move(disposable_list);

    EXPECT_TRUE(list == reference_list);
}

TEST(List, Move_constructor)
{
    List<int> reference_list;
    reference_list.insert(0, 256);
    reference_list.insert(0, 512);
    List<int> disposable_list = reference_list;

    List<int> list = std::move(disposable_list);

    EXPECT_TRUE(list == reference_list);
}

