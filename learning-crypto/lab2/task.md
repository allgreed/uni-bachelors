Zaprogramować szyfrowanie i odszyfrowywanie wiadomości przy użyciu szyfru Vigenere'a. Zakładamy, że tekst jawny jest ciągiem małych liter bez spacji, cyfr i znaków przestankowych. Taki tekst jawny trzeba przygotować z realnie dostępnego tekstu za pomocą odpowiedniego narzędzia.
Program o nazwie vigenere powinien umożliwiać wywołanie z linijki rozkazowej z następującymi opcjami:
-p (przygotowanie tekstu jawnego do szyfrowania),
-e (szyfrowanie),
-d (odszyfrowywanie),
-k (kryptoanaliza wyłącznie w oparciu o kryptogram)

Nazwy tych plików są identyczne jak w poprzednim zadaniu:
plain.txt: plik z tekstem jawnym,
crypto.txt: plik z tekstem zaszyfrowanym,
decrypt.txt: plik z tekstem odszyfrowanym,
key.txt: plik zawierający klucz,
orig.txt: oryginalny tekst, przed przygotowaniem do szyfrowania
key-crypto.txt: plik z kluczem znalezionym w wyniku kryptoanalizy

Treścią zadania jest w zasadzie kryptoanaliza. Nie należy oczekiwać zadowalających wyników jeśli kryptogramy są krótkie. Jednak teksty języka naturalnego o długości setek i więcej znaków, np. zaszyfrowane artykuły prasowe, dadzą się rutynowo odszyfrować bez znajomości klucza.

Literatura: https://inf.ug.edu.pl/~amb/krypto-lab/Vigenere.html
https://andystabler.co.uk/blog/cryptanalysis-vigenere-cipher/

Wskazówki:
    - przesunięcia o minimum 10

Restrykcje:
    - klucz: 6-12 liczb
    - tekst: 1000 znaków

Pytania:
- format klucza? po prostu liczby po spacjach? hyyyba...
- jaka nazwa pliku wykonywalnego? strzelam, że vigenere
- jakie nazwy plików ostatecznie? strzelam, że tak jak w treści
