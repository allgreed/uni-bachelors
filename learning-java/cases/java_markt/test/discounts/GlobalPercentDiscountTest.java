package discounts;

import main.OrderTotal;
import main.Product;
import org.junit.Assert;
import org.junit.Test;

import java.util.ArrayList;

public class GlobalPercentDiscountTest
{

    @Test
    public void applyApplicable()
    {
        ArrayList<Product> productList = new ArrayList<>();
        productList.add(new Product("0", "test0", 330));

        OrderTotal initialTotal = new OrderTotal(productList);
        Discount discount = new GlobalPercentDiscount();

        OrderTotal actual = discount.apply(initialTotal);

        Assert.assertEquals(initialTotal.getProductList(), actual.getProductList());
        Assert.assertEquals(313.5, actual.getPrice(), 0.01);
    }

    @Test
    public void applyNonApplicable()
    {
        ArrayList<Product> productList = new ArrayList<>();
        productList.add(new Product("0", "test0", 290));

        OrderTotal initialTotal = new OrderTotal(productList);
        Discount discount = new GlobalPercentDiscount();

        OrderTotal actual = discount.apply(initialTotal);

        Assert.assertEquals(initialTotal.getProductList(), actual.getProductList());
        Assert.assertEquals(290, actual.getPrice(), 0.01);
    }
}