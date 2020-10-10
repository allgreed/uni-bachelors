package main;

import discounts.Discount;
import discounts.NullDiscount;

import java.util.Collections;
import java.util.Comparator;
import java.util.List;

public class JavaMarkt
{
    private Discount discount;
    private ProductCart productCart;

    public JavaMarkt()
    {
        this.productCart = new ProductCart(Collections.emptyList());
        setDiscount(null);
    }

    public void add(Product product)
    {
        this.productCart.add(product);
    }

    public void setDiscount(Discount discount)
    {
        if (discount == null)
        {
            this.discount = new NullDiscount();
        }
        else
        {
            this.discount = discount;
        }
    }

    public List<Product> getAllProducts()
    {
        OrderTotal total = this.getOrderTotal();
        return total.getProductList();
    }

    public double getTotalPrice()
    {
        OrderTotal total = this.getOrderTotal();
        return total.getPrice();
    }

    public Product getCheapestProduct()
    {
        return this.getManyCheapestProducts(1).get(0);
    }

    public Product getMostExpensiveProduct()
    {
        return this.getManyMostExpensiveProducts(1).get(0);
    }

    public List<Product> getManyCheapestProducts(int count)
    {
        return this.productCart.getManyCheapestProducts(count);
    }

    public List<Product> getManyMostExpensiveProducts(int count)
    {
        return this.productCart.getManyMostExpensiveProducts(count);
    }

    private OrderTotal getOrderTotal()
    {
        OrderTotal totalWithoutDiscounts = new OrderTotal(this.productCart.getProductList());
        return this.discount.apply(totalWithoutDiscounts);
    }
}
