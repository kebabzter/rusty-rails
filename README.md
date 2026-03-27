<pre>
██████╗ ██╗   ██╗███████╗████████╗██╗   ██╗     ██████╗  █████╗ ██╗██╗     ███████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝╚██╗ ██╔╝     ██╔══██╗██╔══██╗██║██║     ██╔════╝
██████╔╝██║   ██║███████╗   ██║    ╚████╔╝█████╗██████╔╝███████║██║██║     ███████╗
██╔══██╗██║   ██║╚════██║   ██║     ╚██╔╝ ╚════╝██╔══██╗██╔══██║██║██║     ╚════██║
██║  ██║╚██████╔╝███████║   ██║      ██║        ██║  ██║██║  ██║██║███████╗███████║
╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝      ╚═╝        ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚══════╝╚══════╝
</pre>

Welcome to the **rusty-rails** project!

This project has been made during the "Tracks in the dark" mini hackathon.

The idea of the project is to emulate trains coming and exiting a train station and how
automation can be used to direct multiple trains to the correct "lane" without having them crash.

The project is built using 100% rust and **INSANE** programming skills. *(the code quality is questionable to be honest)*

## How-to:

Currently the way the project is run is by using `cargo run main.rs`
We plan on compiling an executable file for the future *(as of writing this readme it is is too late and I am too lazy to do it right now)*

Once the application is running you are greeted!

`Press Enter`

After that you are asked if you would like to run a premade or a custom simulation. Type `P` for premade or `C` for custom

### Premade
If you chose premade, well, the simulation is just going to run and that's it. (you can Ctrl-C out of it because it won't end otherwise until memory runs out)

### Custom
If you chose custom you have to input how many trains you would like the simulation to have.

> Have in mind that the current perimeter of the lanes and the length of the rails is hardcoded to 7 lanes with length of 60

You will be prompted `n` number of times *(depending on how many trains you chose)*

The format for adding a train is the following

> Assuming you chose n=2 or to have 2 trains.

```
[direction (left/right)] [from (lane 0-6)] [to (lane 0-6)] [speed (1-3)] [eta (0-60)] [wagons (0-8)]

## Example input

l 2 4 3 30 6
l 6 5 2 30 4
```

This example input will result in 2 trains being created.
 - The first coming from the `left` `2`nd lane and going to the `4`th lane with a speed of `3`, eta until switch of lane = `30` and `6` wagons.
 - The second coming from the `left` `6`th lane and going to the `5`th lane with a speed of `2`, eta until switch of lane = `30` and `4` wagons.

 > eta and direction are pretty much useless right now so don't pay too much attention to them.


The simulation is just going to run and that's it. (you can Ctrl-C out of it because it won't end otherwise until memory runs out)


*(P.S prior to this project both of us have had exactly 0 experience with rust. The only AI used for this project is to search up how rust works. No code has been pasted 100% authentic human-made code. Also this is still WIP so yeah.. :D)*

