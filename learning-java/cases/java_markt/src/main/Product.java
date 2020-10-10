package main;

import java.util.Comparator;
import java.util.Objects;

public class Product implements Comparable<Product>
{
    private final String code;
    private final String name;
    private final double price;

    public Product(String code, String name, double price)
    {
        this.code = code;
        this.name = name;
        this.price = price;
    }

    public String getCode()
    {
        return code;
    }

    public String getName()
    {
        return name;
    }

    public double getPrice()
    {
        return price;
    }

    @Override
    public boolean equals(Object other)
    {
        if (!(other instanceof Product))
        {
            return false;
        }

        Product otherProduct = (Product) other;

        return Objects.equals(this.code, otherProduct.code)
            && Objects.equals(this.price, otherProduct.price)
            && Objects.equals(this.name, otherProduct.name);
    }

    @Override
    public int compareTo(Product product)
    {
        // This is an UB implementation just for the code to compile
        return 0;
    }

    public String toString()
    {
        StringBuilder stringBuilder = new StringBuilder();

        stringBuilder.append(this.name);
        stringBuilder.append(" - ");
        stringBuilder.append(this.price);
        stringBuilder.append(" [");
        stringBuilder.append(this.code);
        stringBuilder.append("]");

        return stringBuilder.toString();
    }
}

class PriceAscendingComparator implements Comparator<Product>
{
    @Override
    public int compare(Product lhs, Product rhs)
    {
        return Double.compare(lhs.getPrice(), rhs.getPrice());
    }
}

class NameAscendingComparator implements Comparator<Product>
{
    @Override
    public int compare(Product lhs, Product rhs)
    {
        return lhs.getName().compareTo(rhs.getName());
    }
}

class PriceThanNameAscendingComparator implements Comparator<Product>
{
    private final Comparator<Product> priceAscendingComparator = new PriceAscendingComparator();
    private final Comparator<Product> nameAscendingComparator = new NameAscendingComparator();

    @Override
    public int compare(Product lhs, Product rhs)
    {
        int priceComparison = priceAscendingComparator.compare(lhs, rhs);

        if (priceComparison == 0) // prices are equal
        {
            return nameAscendingComparator.compare(lhs, rhs);
        }
        else
        {
            return priceComparison;
        }
    }
}
