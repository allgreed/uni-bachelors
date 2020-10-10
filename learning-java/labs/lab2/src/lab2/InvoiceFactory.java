package lab2;

import lab2.utils.NowDateProvider;
import lab2.utils.UniqueIdProvider;

import java.util.Collection;
import java.util.Date;

public class InvoiceFactory
{
    private final UniqueIdProvider idProvider = new UniqueIdProvider();
    private final NowDateProvider nowProvider = new NowDateProvider();

    public Invoice createReceipt(Collection<Product> products, Buyer buyer, Date goodsSoldDate)
    {
        return new Invoice(products, buyer, goodsSoldDate, this.idProvider, this.nowProvider);
    }
}
