package discounts;

import main.OrderTotal;
import main.Product;
import main.ProductCart;

public class XForThePriceOfYDiscount extends Discount
{
    private final int triggerProductCount;
    private final int freeProductCount;

    public XForThePriceOfYDiscount(int X, int Y)
    {
        this.triggerProductCount = X;
        this.freeProductCount = X - Y;
    }

    @Override
    public OrderTotal apply(OrderTotal inputOrderTotal)
    {
        double newPrice = inputOrderTotal.getPrice();

        if (inputOrderTotal.getProductList().size() >= triggerProductCount)
        {
            ProductCart cart = new ProductCart(inputOrderTotal.getProductList());
            newPrice -= cart.getManyCheapestProducts(this.freeProductCount)
                    .stream()
                    .map(Product::getPrice)
                    .reduce(0.0, Double::sum)
                    ;
        }

        return new OrderTotal(newPrice, inputOrderTotal.getProductList());
    }
}
