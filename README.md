<h1>
    <img src="docs/img/icon-64.png" height=32/>
    What-Weather: Weather Reporter for Command Line
</h1>

A simple tool that fetches weather report from external service and presents it in a human-readable format.

# Terms of use
Even though this is a toy project a note is appropriate - this program uses external API's and users should adhere to their terms of use:
- [Open-Meteo](https://open-meteo.com/en/pricing) - weather service 10k calls/day for free
- [ip-api](https://ip-api.com/docs/legal) - geolocation service, 45 calls/minute for free

# Installation
Install the program directly from github using [Rust and Cargo](https://www.rust-lang.org/tools/install):
```
cargo install --git https://github.com/jazalewski1/what-weather
```
or clone this repository and install manually:
```
git clone https://github.com/jazalewski1/what-weather.git
cd what-weather
cargo install --path .
```

# Usage
For all possible usage options run:
```
what-weather --help
```

Report current weather:
```
what-weather now
```

Report forecast for 3 days in specific location:
```
what-weather --coords="48.8584,2.2944" forecast --days=3
```

Report past weather as a list of selected attributes:
```
what-weather past --days=5 --list="temperature,humidity"
```
