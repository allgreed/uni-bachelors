### Założenia i zależności
#### Spójność

Zgodnie z teorią CAP system rozproszony musi zapewniać jedną cechę spośród spójności i dostępności w przypadku podziału sieci. Aplikacja K0S zapewnia spójność kosztem dostępności, ponieważ sieć łącząca węzły klastra będzie względnie niezawodna. Zapewnianie dostępności wiązałoby się z dodatkowym kosztem pracy zespołu deweloperskiego, a nie dostarczałoby wartości klientowi.

