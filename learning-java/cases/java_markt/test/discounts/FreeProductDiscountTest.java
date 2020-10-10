package discounts;

import main.OrderTotal;
import main.Product;
import org.junit.Assert;
import org.junit.Test;

import java.util.ArrayList;

public class FreeProductDiscountTest
{
    @Test
    public void applyApplicable()
    {
        ArrayList<Product> productList = new ArrayList<>();
        productList.add(new Product("0", "test0", 210));

        ArrayList<Product> expectedProductList = new ArrayList<>();
        expectedProductList.add(new Product("0", "test0", 210));
        expectedProductList.add(new Product("-1", "Company Mug", 10.0));

        OrderTotal initialTotal = new OrderTotal(productList);
        Discount discount = new FreeProductDiscount();

        OrderTotal actual = discount.apply(initialTotal);

        Assert.assertEquals(expectedProductList, actual.getProductList());
        Assert.assertEquals(210, actual.getPrice(), 0.01);
    }

    @Test
    public void applyNonApplicable()
    {
        ArrayList<Product> productList = new ArrayList<>();
        productList.add(new Product("0", "test0", 190));

        OrderTotal initialTotal = new OrderTotal(productList);
        Discount discount = new FreeProductDiscount();

        OrderTotal actual = discount.apply(initialTotal);

        Assert.assertEquals(initialTotal.getProductList(), actual.getProductList());
        Assert.assertEquals(190, actual.getPrice(), 0.01);
    }
}