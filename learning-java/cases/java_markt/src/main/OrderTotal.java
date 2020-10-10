package main;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

public class OrderTotal
{
    private final double price;
    private final List<Product> productCollection;

    public OrderTotal()
    {
        this.price = 0;
        this.productCollection = new ArrayList<>();
    }

    public OrderTotal(double price, List<Product> productCollection)
    {
        this.price = price;
        this.productCollection = productCollection;
    }

    public OrderTotal(List<Product> productCollection)
    {
        this.productCollection = productCollection;
        this.price = this.productCollection
                .stream()
                .map(Product::getPrice)
                .reduce(0.0, Double::sum);
    }

    public double getPrice()
    {
        return price;
    }

    public List<Product> getProductList()
    {
        return productCollection;
    }

    @Override
    public boolean equals(Object other)
    {
        if (!(other instanceof OrderTotal))
        {
            return false;
        }

        OrderTotal otherOrderTotal = (OrderTotal) other;

        return Objects.equals(this.productCollection, otherOrderTotal.productCollection)
                && Objects.equals(this.price, otherOrderTotal.price);
    }
}
