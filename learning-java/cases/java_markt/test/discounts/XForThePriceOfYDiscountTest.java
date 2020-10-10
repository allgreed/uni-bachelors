package discounts;

import main.OrderTotal;
import main.Product;
import org.junit.Assert;
import org.junit.Test;

import java.util.ArrayList;

import static org.junit.Assert.*;

public class XForThePriceOfYDiscountTest
{
    @Test
    public void applyApplicable()
    {
        ArrayList<Product> productList = new ArrayList<>();
        productList.add(new Product("1", "test1", 100));
        productList.add(new Product("1", "test1", 100));
        productList.add(new Product("1", "test1", 100));

        OrderTotal initialTotal = new OrderTotal(productList);
        Discount discount = new XForThePriceOfYDiscount(3,2);

        OrderTotal actual = discount.apply(initialTotal);

        Assert.assertEquals(200, actual.getPrice(), 0.01);
        Assert.assertEquals(initialTotal.getProductList(), actual.getProductList());
    }

    @Test
    public void applyNonApplicable()
    {
        ArrayList<Product> productList = new ArrayList<>();
        productList.add(new Product("1", "test1", 100));
        productList.add(new Product("1", "test1", 100));
        productList.add(new Product("1", "test1", 100));

        OrderTotal initialTotal = new OrderTotal(productList);
        Discount discount = new XForThePriceOfYDiscount(4,3);

        OrderTotal actual = discount.apply(initialTotal);

        Assert.assertEquals(300, actual.getPrice(), 0.01);
        Assert.assertEquals(initialTotal.getProductList(), actual.getProductList());
    }
}