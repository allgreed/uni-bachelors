package main;

import java.util.List;

public class ProductListConsoleView
{
    static void printProductsToConsole(List<Product> products)
    {
        System.out.println("Cart contains following products:");

        for (Product product : products)
        {
            System.out.println(product);
        }
    }
}
