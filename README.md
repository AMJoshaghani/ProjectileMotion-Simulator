# ProjectileMotion-Simulator
Using Rust & Webassembly (powered by [rustwasm](https://github.com/rustwasm)!) to simulate the motion of a projectile with bouncing and damping accounted for.
<br />

![picture](https://s6.uupload.ir/files/screenshot_2025-07-03_at_21-03-18_projectile_motion_np4.png)
# Build and run
Build wasm by 
```
$ wasm-pack build --release
```
then `cd` into `www/` and run
```
$ npm install
```
followed by 
```
$ npm run start
```
and a local server should start afterwards.
