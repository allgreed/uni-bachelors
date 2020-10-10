package main;

import discounts.Discount;
import org.junit.Assert;
import org.junit.Test;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.List;


public class JavaMarktTest
{
    @Test
    public void getAllProductsWhenMarkIsEmpty()
    {
        JavaMarkt javaMarkt = new JavaMarkt();

        Collection<Product> actual = javaMarkt.getAllProducts();

        Assert.assertEquals(new ArrayList<>(), actual);
    }

    @Test
    public void getAllProducts()
    {
        JavaMarkt javaMarkt = new JavaMarkt();
        javaMarkt.add(new Product("2", "test2", 500.0));
        javaMarkt.add(new Product("0", "test0", 20.0));
        javaMarkt.add(new Product("3", "test3", 100.0));
        javaMarkt.add(new Product("1", "test1", 200.0));
        List<Product> expected = new ArrayList<>(Arrays.asList(
            new Product("0", "test0", 20.0),
            new Product("3", "test3", 100.0),
            new Product("1", "test1", 200.0),
            new Product("2", "test2", 500.0)
        ));

        List<Product> actual = javaMarkt.getAllProducts();

        Assert.assertEquals(expected, actual);
    }

    @Test
    public void applyDiscount()
    {
        class SampleDiscount extends Discount
        {
            @Override
            public OrderTotal apply(OrderTotal inputOrderTotal)
            {
                double newPrice = inputOrderTotal.getPrice() - 5.0;
                return new OrderTotal(newPrice, inputOrderTotal.getProductList());
            }
        }

        JavaMarkt javaMarkt = new JavaMarkt();
        javaMarkt.setDiscount(new SampleDiscount());
        javaMarkt.add(new Product("0", "test0", 10.0));

        double actual = javaMarkt.getTotalPrice();

        Assert.assertEquals(5, actual, 0.01);
    }

    @Test
    public void applyNullDiscount()
    {
        JavaMarkt javaMarkt = new JavaMarkt();
        javaMarkt.setDiscount(null);
        javaMarkt.add(new Product("0", "test0", 10.0));

        double actual = javaMarkt.getTotalPrice();

        Assert.assertEquals(10.0, actual, 0.01);
    }

    @Test
    public void getTotalPrice()
    {
        JavaMarkt javaMarkt = new JavaMarkt();
        javaMarkt.add(new Product("0", "test0", 20.0));
        javaMarkt.add(new Product("1", "test1", 200.0));
        javaMarkt.add(new Product("2", "test2", 100.0));

        double actual = javaMarkt.getTotalPrice();

        Assert.assertEquals(320.0, actual, 0.01);
    }

    @Test
    public void getTotalEmptyPrice()
    {
        JavaMarkt javaMarkt = new JavaMarkt();

        double actual = javaMarkt.getTotalPrice();

        Assert.assertEquals(0.0, actual, 0.01);
    }

    @Test
    public void getCheapestProduct()
    {
        JavaMarkt javaMarkt = new JavaMarkt();
        javaMarkt.add(new Product("2", "test2", 500.0));
        javaMarkt.add(new Product("0", "test0", 20.0));
        javaMarkt.add(new Product("3", "test3", 100.0));
        javaMarkt.add(new Product("1", "test1", 200.0));

        Product actual = javaMarkt.getCheapestProduct();

        Product expected = new Product("0", "test0", 20.0);
        Assert.assertEquals(expected, actual);
    }

    @Test
    public void getMostExpensiveProduct()
    {
        JavaMarkt javaMarkt = new JavaMarkt();
        javaMarkt.add(new Product("2", "test2", 500.0));
        javaMarkt.add(new Product("0", "test0", 20.0));
        javaMarkt.add(new Product("3", "test3", 100.0));
        javaMarkt.add(new Product("1", "test1", 200.0));

        Product actual = javaMarkt.getMostExpensiveProduct();

        Product expected = new Product("2", "test2", 500.0);
        Assert.assertEquals(expected, actual);
    }

    @Test
    public void getManyOrSingleProductWhenMarktIsEmpty()
    {
        JavaMarkt javaMarkt = new JavaMarkt();

        try
        {
            javaMarkt.getManyCheapestProducts(3);
            Assert.fail( "My method didn't throw when I expected it to" );
        }
        catch (IndexOutOfBoundsException expectedException){}

        try
        {
            javaMarkt.getManyMostExpensiveProducts(5);
            Assert.fail( "My method didn't throw when I expected it to" );
        }
        catch (IndexOutOfBoundsException expectedException){}

        try
        {
            javaMarkt.getCheapestProduct();
            Assert.fail( "My method didn't throw when I expected it to" );
        }
        catch (IndexOutOfBoundsException expectedException){}

        try
        {
            javaMarkt.getMostExpensiveProduct();
            Assert.fail( "My method didn't throw when I expected it to" );
        }
        catch (IndexOutOfBoundsException expectedException){}
    }

    @Test
    public void getManyCheapestProducts()
    {
        JavaMarkt javaMarkt = new JavaMarkt();
        javaMarkt.add(new Product("2", "test2", 500.0));
        javaMarkt.add(new Product("0", "test0", 20.0));
        javaMarkt.add(new Product("3", "test3", 100.0));
        javaMarkt.add(new Product("1", "test1", 200.0));

        List<Product> expected = new ArrayList<>(Arrays.asList(
            new Product("0", "test0", 20.0),
            new Product("3", "test3", 100.0)
        ));

        List<Product> actual = javaMarkt.getManyCheapestProducts(2);

        Assert.assertEquals(expected, actual);
    }

    @Test
    public void getManyMostExpensiveProducts()
    {
        JavaMarkt javaMarkt = new JavaMarkt();
        javaMarkt.add(new Product("2", "test2", 500.0));
        javaMarkt.add(new Product("0", "test0", 20.0));
        javaMarkt.add(new Product("3", "test3", 100.0));
        javaMarkt.add(new Product("1", "test1", 200.0));

        List<Product> expected = new ArrayList<>(Arrays.asList(
            new Product("1", "test1", 200.0),
            new Product("2", "test2", 500.0)
        ));

        List<Product> actual = javaMarkt.getManyMostExpensiveProducts(2);

        Assert.assertEquals(expected, actual);
    }
}