# Сбор данных о погодных условиях

## Оборудование
<ul>
<li> Датчик температуры и влажности Dht-22
<li>Raspbery py4 B
</ul>
список в дальнейшем будет пополняться

## Инструкция по запуску
<ul>
<li>Устанавливаем на расбери ОС <a href = "https://www.raspberrypi.com/software/">инструкция с офф сайта</a>
<li>устанавливаем язык программирования rust
<pre>
<code>
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf -v -4 | sh

export PATH="$HOME/.cargo/bin:$PATH"
</code>
</pre>
<li> Клонируем репозиторий командой git clone
<li>Собираем проект командой cargo build и запускаем cargo run
<li> подключаем датчик как <a href = "https://habrastorage.org/storage2/a84/bd3/77a/a84bd377a9ad2d3bbe7376a0b89418d0.jpg">тут</a> 
</ul>
