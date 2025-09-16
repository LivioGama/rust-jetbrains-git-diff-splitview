Very important!

Try to not give up so easily and giving me instructions.Take initiatives and do the next sections until you succeed.

Never recreate long files (more than 40 lines) from scratch when they are broken, instead fix them.

Never generate documentation unless explicitly asked for. But if you do, don't start it before all the code is working, running and approved explicitely by me.

"sn" means "still not working try another method"

Never change the implemented or plugged example for some dummy sample.

Never generate example code or demo components.

Never do something different from what I ask, for example, removing a feature for simplicity.

# Rust

Never run `cargo run`, run `cargo check` instead. I always watch files.

Try to keep your files short (under 250 lines), and the architecture modular.

# NPM TS JS

Also in general, if a port you need is blocked, don't hesitate to kill it with `kill $(lsof -t -i:PORT_NUMBER)`.

Never run `bun run dev` or `bun run build` on NextJS, use `tsc` to detect typescript errors.

When running `bun run dev` or finishing a task where the project is already running, always open the localhost project url in the browser. If you have your own as a plugin, prefer it.

At the end of each task, pass lint fix. Then check all typescript errors by building and fix them. And then a final lint fix
