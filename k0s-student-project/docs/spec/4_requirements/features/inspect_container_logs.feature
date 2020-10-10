Funkcja: Sprawdź logi kontenerów

  Scenariusz: Zobacz listę wszystkich użytych kontenerów
    Zakładając, że użytkownik jest zalogowany
    Kiedy zdecyduje się podejrzeć logi kontenerów
    Wtedy powinien zobaczyć listę wszystkich kiedykolwiek aktywnych kontenerów
    Oraz być wstanie wybrać do inspekcji konkretny kontener

  Scenariusz: Zobacz szczegóły wybranego kontenera
    Zakładając, że użytkownik jest zalogowany
    Oraz jest na stronie logów kontenerów
    Oraz uruchomił już wcześniej jakiś kontener
    Kiedy podejrzy konkretny kontener
    Wtedy powinien zobaczyć jego logi
    Oraz jego stan

  Scenariusz: Próba podejrzenia nie istniejącego kontenera
    Zakładając, że użytkownik jest zalogowany
    Oraz jest na stronie logów kontenerów
    Kiedy podejrzy konkretny kontener
    Oraz ten kontener zostanie zweryfikowany jako nieistniejący
    Wtedy użytkownik powinien zostać o tym poinformowany
    Oraz przeniesiony do listy wszystkich kontenerów
