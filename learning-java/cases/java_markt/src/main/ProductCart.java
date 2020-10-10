package main;

import utilities.SortedProductList;

import java.util.List;

public class ProductCart
{
    private final SortedProductList productList;

    public ProductCart(List<Product> productList)
    {
        this.productList = new SortedProductList(new PriceThanNameAscendingComparator());
        this.productList.addAll(productList);
    }

    public void add(Product product)
    {
        this.productList.add(product);
    }

    public List<Product> getManyCheapestProducts(int count)
    {
        return this.productList.subList(0, count);
    }

    public List<Product> getManyMostExpensiveProducts(int count)
    {
        return this.productList.subList(productList.size() - count, productList.size());
    }
    public List<Product> getProductList()
    {
        return (List<Product>) this.productList;
    }
}
