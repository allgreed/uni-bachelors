package discounts;

import main.OrderTotal;


public class NullDiscount extends Discount
{
    @Override
    public OrderTotal apply(OrderTotal inputOrderTotal)
    {
        return inputOrderTotal;
    }
}
