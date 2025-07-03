import * as wasm from "projectile-motion";

const fps = 60;
let angle, velocity, damping_constant, b, dt = 0;
let coordination_list = [];
const err = document.querySelector("#err");
const canvas = document.querySelector("#canvas");
const ctx = canvas.getContext('2d');
ctx.transform(1, 0, 0, -1, 0, canvas.height);
draw_line();

function start(){
    angle = document.querySelector("#angle").value || 0;
    velocity = document.querySelector("#velocity").value || 0;
    damping_constant = document.querySelector("#damping_constant").value || 0;
    if (!angle || !velocity || !damping_constant){
        err.style.display = 'block';
        return;
    } else {
        err.style.display = 'none';
    }
    dt = 0
    if (b)
        b = null
    b = wasm.Ball.new(0, 85, angle, velocity, damping_constant);
    animate();
}
function animate() {
    draw_line();
    const x = b.get_x();
    const y = b.get_y();

    ctx.fillStyle = "rgba(255, 255, 255, 0.1)";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = "black";
    ctx.font = "15px Arial";
    ctx.fillText("⚽", x, y);

    const step = .1 / fps;
    b.tick(dt);
    dt += step;

    if (!b.stopped()) {
        setTimeout(() => {
            requestAnimationFrame(animate);
        }, 1000 / fps);
    } else {
        let new_coordination = [velocity, angle, x, y];
        coordination_list.indexOf(new_coordination) === -1 ? coordination_list.push(new_coordination) : null
        write_info();
    }
}

function draw_line(){
    ctx.beginPath();
    ctx.moveTo(0, 80);
    ctx.lineTo(2000, 80);
    ctx.strokeStyle = "grey";
    ctx.stroke();
}

function write_info(){
    ctx.save();
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    coordination_list.forEach(function (e){
        ctx.fillText('vel:', e[2], e[3] + 940);
        ctx.fillText(e[0], e[2], e[3] + 960);
        ctx.fillText('ang:', e[2], e[3] + 980);
        ctx.fillText(e[1], e[2], e[3] + 1000);
    });
    ctx.restore();
}

document.querySelector('#begin').addEventListener('click', start);