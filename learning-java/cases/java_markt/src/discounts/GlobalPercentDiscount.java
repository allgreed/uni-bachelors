package discounts;

import main.OrderTotal;

public class GlobalPercentDiscount extends Discount
{
    private final double triggerPrice;
    private final int discountPercentage;

    public GlobalPercentDiscount(double triggerPrice, int discountPercentage)
    {
        this.triggerPrice = triggerPrice;
        this.discountPercentage = discountPercentage;
    }

    public GlobalPercentDiscount()
    {
        this.triggerPrice = 300;
        this.discountPercentage = 5;
    }

    @Override
    public OrderTotal apply(OrderTotal inputOrderTotal)
    {
        double newPrice = inputOrderTotal.getPrice();

        if (inputOrderTotal.getPrice() > triggerPrice)
        {
            double discountMultiplier = (double) (100 - this.discountPercentage) / 100;
            newPrice *= discountMultiplier;
        }

        return new OrderTotal(newPrice, inputOrderTotal.getProductList());
    }
}
