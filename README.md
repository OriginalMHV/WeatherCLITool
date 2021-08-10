# Weather CLI Tool
![Made with love in Rust](https://madewithlove.now.sh/af?heart=true&colorB=%23ff8800&template=for-the-badge&text=Rust)

CLI tool that shows you everything you need to know about the weather.
First thing to set the local .env variable to your [OpenWeatherMap](https://openweathermap.org/api) API key.
Then run it with Cargo:

``` Cargo run City Country Code ```

Example for the city Haugesund in Norway

``` Cargo run Haugesund NO ```

This was a small project to learn Rust and how to use API's. I'll update it with better solutions when I get the hang of the Rust language.



#### Todo's
- [X] Get the current weather forecast working with API
- [X] Able to use CL to get weather and print it out
- [ ] Get the API key to work with .env <- Current problem
- [ ] Add options for the input (e.g., future weather forecasts)
- [ ] Add settings to de-cluster the output (e.g., only temp)



