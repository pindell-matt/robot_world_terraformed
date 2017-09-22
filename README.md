# Robot World Terraformed
### A Basic CRUD App using [Rust](https://www.rust-lang.org/en-US/), [Rocket](https://rocket.rs/), & [Elm](http://elm-lang.org/)
Inspired by the [Turing School Project - Robot World](https://github.com/turingschool-examples/robot-world)

**Still a work-in-progress, need to finish Elm app and seed database w/ Robots**

[ assumes you have Rust, Elm, and PostgreSQL installed ]

Setting up the database:
```
echo DATABASE_URL=postgres://username:password@localhost/robot_world_terraformed > .env
diesel migration run
```

Starting the App:
```
cd app
yarn run start
```
