package lab2;

import lab2.utils.NowDateProvider;
import lab2.utils.UniqueIdProvider;

import java.util.*;
import java.util.stream.Collectors;

public class Invoice
{
    private final Collection<InvoiceItem> items;
    private final Buyer buyer;
    private final int id;
    private final Date issueDate;
    private final Date goodsSoldDate;

    private Date paymentReceivedDate;
    private boolean isPaymentReceiveDateSet = false;

    public Invoice(Collection<Product> products, Buyer buyer, Date goodsSoldDate, UniqueIdProvider idProvider, NowDateProvider nowProvider)
    {
        // TODO: get rid of .collect.entrySet.stream()

        // distinct -> map(x -> arr.filter(x).count())
        this.items = products
                .stream()
                .collect(Collectors.groupingBy(s -> s, Collectors.counting()))
                .entrySet()
                .stream()
                .map(entry -> new InvoiceItem(entry.getKey(), entry.getValue().intValue()))
                .collect(Collectors.toList())
                ;

        this.buyer = buyer;
        this.goodsSoldDate = goodsSoldDate;

        this.id = idProvider.getNextId();
        this.issueDate = nowProvider.getCurrentDate();
    }

    public Invoice(Collection<Product> products, Buyer buyer, Date goodsSoldDate, UniqueIdProvider idProvider, NowDateProvider nowProvider, Date paymentReceivedDate)
    {
        this(products, buyer, goodsSoldDate, idProvider, nowProvider);
        this.setPaymentReceivedDate(paymentReceivedDate);
    }

    public double getTotal()
    {
        return items
            .stream()
            .map(InvoiceItem::getTotal)
            .reduce(0.0, Double::sum)
            ;
    };

    public void setPaymentReceivedDate(Date paymentReceivedDate)
    {
        if (!this.isPaymentReceiveDateSet)
        {
            this.paymentReceivedDate = paymentReceivedDate;
            this.isPaymentReceiveDateSet = true;
        }
    }

    public Collection<InvoiceItem> getItems()
    {
        return items;
    }

    public Buyer getBuyer()
    {
        return buyer;
    }

    public int getId()
    {
        return id;
    }

    public Date getIssueDate()
    {
        return issueDate;
    }

    public Date getGoodsSoldDate()
    {
        return goodsSoldDate;
    }

    public Date getPaymentReceivedDate()
    {
        return paymentReceivedDate;
    }
}
