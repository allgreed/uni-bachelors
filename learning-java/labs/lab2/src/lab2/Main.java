package lab2;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Date;

public class Main {

    public static void main(String[] args) {
        InvoiceFactory receiptFactory = new InvoiceFactory();

        Invoice invoice = receiptFactory.createReceipt(new ArrayList<Product>(Arrays.asList(
           new Product(30, "elo"),
           new Product(30, "elo")
        )), new Buyer(), new Date());
    }
}
