package main;

public class Main {

    public static void main(String[] args)
    {
	    JavaMarkt javaMarkt = new JavaMarkt();

        javaMarkt.add(new Product("2", "Mokasyny", 200));
		javaMarkt.add(new Product("1", "Skarpety", 50));
        javaMarkt.add(new Product("3", "Jakiś lizak", 2));
        javaMarkt.add(new Product("0", "Kartofle", 50));

		ProductListConsoleView.printProductsToConsole(javaMarkt.getAllProducts());
    }
}
