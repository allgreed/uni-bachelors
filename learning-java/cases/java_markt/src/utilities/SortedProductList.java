package utilities;

import main.Product;

import java.util.ArrayList;
import java.util.Comparator;

public class SortedProductList extends ArrayList<Product>
{
    private final Comparator<Product> productComparator;

    public SortedProductList(Comparator<Product> productComparator)
    {
        this.productComparator = productComparator;
    }

    @Override
    public boolean add(Product product)
    {
        boolean returnValue = super.add(product);

        this.sort(this.productComparator);

        return returnValue;
    }
}
