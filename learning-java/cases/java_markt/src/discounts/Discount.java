package discounts;

import main.OrderTotal;


abstract public class Discount
{
    abstract public OrderTotal apply(OrderTotal inputOrderTotal);
}
