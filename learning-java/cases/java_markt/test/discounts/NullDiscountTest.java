package discounts;

import main.OrderTotal;
import main.Product;
import org.junit.Assert;
import org.junit.Test;

import java.util.ArrayList;

import static org.junit.Assert.*;

public class NullDiscountTest
{

    @Test
    public void apply()
    {
        ArrayList<Product> productList = new ArrayList<>();
        productList.add(new Product("0", "test0", 330));

        OrderTotal initialTotal = new OrderTotal(productList);
        Discount discount = new NullDiscount();

        OrderTotal actual = discount.apply(initialTotal);

        Assert.assertEquals(initialTotal, actual);
    }
}