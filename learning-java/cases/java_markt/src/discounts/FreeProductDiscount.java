package discounts;

import main.OrderTotal;
import main.Product;

import java.util.List;

public class FreeProductDiscount extends Discount
{
    private final double triggerPrice;
    private final Product freeProduct;

    public FreeProductDiscount(double triggerPrice, Product freeProduct)
    {
        this.triggerPrice = triggerPrice;
        this.freeProduct = freeProduct;
    }

    public FreeProductDiscount()
    {
        this.triggerPrice = 200;
        this.freeProduct = new Product("-1", "Company Mug", 10.0);
    }

    @Override
    public OrderTotal apply(OrderTotal inputOrderTotal)
    {
        List<Product> newProductCollection = inputOrderTotal.getProductList();

        if (inputOrderTotal.getPrice() > triggerPrice)
        {
           newProductCollection.add(this.freeProduct);
        }

        return new OrderTotal(inputOrderTotal.getPrice(), newProductCollection);
    }
}

