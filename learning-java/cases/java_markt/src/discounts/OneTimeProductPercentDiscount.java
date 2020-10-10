package discounts;

import main.OrderTotal;
import main.Product;

import java.util.stream.Collectors;

public class OneTimeProductPercentDiscount extends Discount
{
    private final Product discountedProduct;
    private final int discountPercentage;

    public OneTimeProductPercentDiscount(Product product, int discountPercentage)
    {
        this.discountedProduct = product;
        this.discountPercentage = discountPercentage;
    }

    public OneTimeProductPercentDiscount(Product product)
    {
        this.discountedProduct = product;
        this.discountPercentage = 30;
    }

    @Override
    public OrderTotal apply(OrderTotal inputOrderTotal)
    {
        double newPrice = inputOrderTotal.getPrice();

        boolean discountedObjectIsPresent = !inputOrderTotal
            .getProductList()
            .stream()
            .filter(item -> item.equals(this.discountedProduct))
            .collect(Collectors.toList())
            .isEmpty()
            ;

        if (discountedObjectIsPresent)
        {
            double discountMultiplier = (double) (this.discountPercentage) / 100;
            newPrice -= this.discountedProduct.getPrice() * discountMultiplier;
        }

        return new OrderTotal(newPrice, inputOrderTotal.getProductList());
    }
}
