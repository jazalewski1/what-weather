# What-Weather user scenarios
*Purpose: this document is intended mostly as a record of planned features during development.*

## Weather attributes
1. Data shall contain following weather attributes with available units:
    - weather type,
    - temperature \[°C, °F\],
    - pressure \[hPa\],
    - humidity \[%\],
    - wind speed \[km/h, m/s, mph, knots\] and direction \[°, compass directions\],
    - cloud coverage \[%\],


## Current weather
1. View text summary of current weather.
    ```
    what-weather
    what-weather now --summary
    ```
1. View attribute list of current weather.
    ```
    what-weather now --list
    ```
1. View attribute list of specified attributes of current weather.
    ```
    what-weather now --list=temperature
    what-weather now --list="pressure,humidity,cloud_coverage"
    ```

## Forecast
1. Forecast shall contain hourly data.

1. View forecast text summary for today.
    ```
    what-weather forecast
    what-weather forecast --summary
    ```

1. View forecast attributes for next few days.
    ```
    what-weather forecast --days=3
    ```

## Past weather
1. Past weather shall contain hourly data.

1. View past weather for few days.
    ```
    what-weather past --days=14
    ```

## Time period
1. View information from time period specified by dates.
    ```
    what-weather period --start="2025-08-01" --end="2025-08-20"
    ```

## Location
1. View information for current location.
    ```
    what-weather <CMD>
    what-weather <CMD> --here
    ```

1. View information for location specified by address.
    ```
    what-weather <CMD> --address="Country, State, City"
    ```

1. View information for location specified by latitude and longitude.
    ```
    what-weather <CMD> --coords="12.345,6.789"
    ```

## Configuration
1. Config file shall be written in JSON/TOML format (TBD), and may contain following parameters:
    - weather provider API key
    - geolocation provider API key
    - location
    - units

1. Use config file in home location if present by default.

1. Specify path config file.
    ```
    what-weather <CMD> --config="/path/to/config.json"
    ```

## Miscellaneous
1. Specify units for specific attributes.
    ```
    what-weather <CMD> --speed_unit=mph
    what-weather <CMD> --temperature_unit=fahrenheit
    ```

1. Specify timezone.
    ```
    what-weather <CMD> --timezone=GMT+2
    ```

1. Output information in JSON or CSV formats.
    ```
    what-weather <CMD> --json
    what-weather <CMD> --csv
    ```

1. Get help message.
    ```
    what-weather --help
    ```
