package lab2;

public class InvoiceItem
{
    private final Product product;
    private final int amount;

    // TODO: tax
    // TODO: gross / netto (jak po angielsku ? net) total

    public InvoiceItem(Product product, int amount)
    {
        this.product = product;
        this.amount = amount;
    }

    public double getTotal()
    {
        return this.product.getPrice() * this.amount;
    }

    public Product getProduct()
    {
        return product;
    }

    public int getAmount()
    {
        return amount;
    }
}
