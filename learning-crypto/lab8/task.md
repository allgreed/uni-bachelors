Program o nazwie elgamal korzysta z pliku elgamal.txt zawierającego liczbę pierwszą p oraz generator g. Testy programu będą wykonane z plikiem o zawartości:

```
1665997633093155705263923663680487185948531888850484859473375695734301776192932338784530163
 170057347237941209366519667629336535698946063913573988287540019819022183488419112350737049
```

Program wywołany z opcją

    -k czyta z powyższego pliku i generuje parę kluczy zapisanych w plikach private.txt oraz public.txt. Każdy klucz składa się z dwu wierszy skopiowanych z powyższego pliku oraz trzeciego wiersza zawierającego odpowiednio wykładnik lub potęgę.
    -e odczytuje klucz publiczny, następnie odczytuje wiadomość z pliku plain.txt i zapisuje zaszyfrowaną wiadomość w pliku crypto.txt. Jeśli warunek m<p nie jest spełniony, sygnalizuje błąd.
    -d odczytuje klucz prywatny i kryptogram, a odszyfrowaną wiadomość zapisuje w pliku decrypt.txt.
    -s odczytuje klucz prywatny, następnie odczytuje wiadomość z pliku message.txt i produkuje podpis, czyli dwa wiersze zapisane do pliku signature.txt.
    -v odczytuje klucz publiczny, wiadomość z pliku message.txt oraz podpis z pliku signature.txt i weryfikuje ten podpis. Wynik weryfikacji (T/N) jest wyświetlany na ekranie oraz jest zapisywany w pliku verify.txt. 

Pliki plain.txt oraz message.txt mogą być identyczne.

Sprawdzenie poprawności programu będzie m.in. zawierało sprawdzenie identyczności plików plain.txt oraz decrypt.txt oraz sprawdzenie poprawności weryfikowania podpisu poprawnego (tzn. signature.txt powstał w opisany wyżej sposób) i niepoprawnego w przeciwnym przypadku.

Program musi wykorzystywać działania arytmetyczne na liczbach kilkusetbitowych, w niektórych językach, np. python, są one dostępne bez dodatkowych bibliotek, w innych, np. Java, konieczne będzie użycie odpowiednich bibliotek.

Program nie ma prawa odczytywać innych plików niż wskazanych w poszczególnych opcjach. 

- do the api
- make it compile

- and tests

- write
