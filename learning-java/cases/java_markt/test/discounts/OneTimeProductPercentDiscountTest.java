package discounts;

import main.OrderTotal;
import main.Product;
import org.junit.Assert;
import org.junit.Test;

import java.util.ArrayList;

public class OneTimeProductPercentDiscountTest
{

    @Test
    public void applyOnce()
    {
        ArrayList<Product> productList = new ArrayList<>();
        productList.add(new Product("0", "test0", 100));

        OrderTotal initialTotal = new OrderTotal(productList);
        Discount discount = new OneTimeProductPercentDiscount(new Product("0", "test0", 100));

        OrderTotal actual = discount.apply(initialTotal);

        Assert.assertEquals(70, actual.getPrice(), 0.01);
        Assert.assertEquals(initialTotal.getProductList(), actual.getProductList());
    }

    @Test
    public void applyToManyOfKind()
    {
        ArrayList<Product> productList = new ArrayList<>();
        productList.add(new Product("0", "test0", 100));
        productList.add(new Product("0", "test0", 100));
        productList.add(new Product("0", "test0", 100));

        OrderTotal initialTotal = new OrderTotal(productList);
        Discount discount = new OneTimeProductPercentDiscount(new Product("0", "test0", 100));

        OrderTotal actual = discount.apply(initialTotal);

        Assert.assertEquals(270, actual.getPrice(), 0.01);
        Assert.assertEquals(initialTotal.getProductList(), actual.getProductList());
    }

    @Test
    public void applyToNoneOfKind()
    {
        ArrayList<Product> productList = new ArrayList<>();
        productList.add(new Product("1", "test1", 100));
        productList.add(new Product("1", "test1", 100));
        productList.add(new Product("1", "test1", 100));

        OrderTotal initialTotal = new OrderTotal(productList);
        Discount discount = new OneTimeProductPercentDiscount(new Product("0", "test0", 5));

        OrderTotal actual = discount.apply(initialTotal);

        Assert.assertEquals(300, actual.getPrice(), 0.01);
        Assert.assertEquals(initialTotal.getProductList(), actual.getProductList());
    }
}