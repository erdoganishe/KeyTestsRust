# KeyTestsRust
Тестування ключів на випадковість за стандартом FIPS-140

Опис завдання:

Реалізація монобітного тесту
Реалізація тесту максимальної довжини серії
Реалізація тесту Поккера
Реалізація тесту довжин серій

Вхідними даними до функцій тестування має бути послідовність довжиною 20 000 бітів. Вихідними даними кожної функції тестування має бути логічне значення (true / false), тест пройдений та не пройдений відповідно. У разі якщо послідовність вдало проходить усі чотири функції тестування, програма має звітувати, що 20 000 бітів є достатньо випадковими. В іншому випадку послідовність бітів відхиляється.


Для запуску програми треба мати встановлену мову rust, перейти в каталозі до папки key_test_run та виконати команду cargo run в cmd.

Список залежностей: 

        Rand 0.8.5;