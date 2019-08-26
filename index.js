import * as chip8 from './pkg/chip8_wasm';
import { memory } from './pkg/chip8_wasm_bg';

const keys = {
    49: 0x1,
    50: 0x2,
    51: 0x3,
    52: 0xC,
    81: 0x4,
    87: 0x5,
    69: 0x6,
    82: 0xD,
    65: 0x7,
    83: 0x8,
    68: 0x9,
    70: 0xE,
    90: 0xA,
    88: 0x0,
    67: 0xB,
    86: 0xF
};

const roms = [
    "15PUZZLE",
    "BLITZ",
    "CONNECT4",
    "HIDDEN",
    "MERLIN",
    "PONG",
    "PUZZLE",
    "TANK",
    "TICTAC",
    "VBRIX",
    "WIPEOFF",
    "BLINKY",
    "BRIX",
    "GUESS",
    "INVADERS",
    "MAZE",
    "MISSILE",
    "PONG2",
    "SYZYGY",
    "TETRIS",
    "UFO",
    "VERS"
];

roms.forEach(rom => {
    const div = document.createElement('div');
    
    div.textContent = rom;
    div.classList.add('option');
    div.addEventListener('click', e => {
        stop();
        document.getElementById('menu').classList.remove('active');
        loadROM(e.target.textContent);
        document.getElementById('rom').textContent = e.target.textContent;        
    });
    
    document.getElementById('menu').appendChild(div);
});

document.getElementById('select').addEventListener('click', () => {
    document.getElementById('menu').classList.toggle('active');
});

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

ctx.fillRect(0, 0, canvas.width, canvas.height);

const loadROM = async rom => {
    const mem = new Uint8Array(memory.buffer, chip8.get_memory(), 4096);
    await fetch(`./roms/${rom}`)
        .then(res => res.arrayBuffer())
        .then(buf => {
            chip8.reset();
            const romData = new Uint8Array(buf);
            for (let i = 0; i < romData.length; i++) {
                mem[i + 0x200] = romData[i];
            }
        })
        .catch(err => console.error(err));
    ctx.fillRect(0, 0, canvas.width, canvas.height);
};

const drawGfx = () => {
    const gfx = new Uint8Array(memory.buffer, chip8.get_display(), 2048);
    const imgData = ctx.createImageData(canvas.width, canvas.height);
    for (let i = 0; i < gfx.length; i++) {
        imgData.data[i * 4 + 0] = gfx[i] ? 255 : 0;
        imgData.data[i * 4 + 1] = gfx[i] ? 255 : 0;
        imgData.data[i * 4 + 2] = gfx[i] ? 255 : 0;
        imgData.data[i * 4 + 3] = 255;
    }
    ctx.putImageData(imgData, 0, 0);
};

let raf = null;

const renderLoop = () => {   
    for (let i = 0; i < 10; i++) {
        chip8.emulate_cycle();
        if (chip8.draw_pending()) {
            drawGfx();
            chip8.unset_draw_flag();
        }
    }
    
    raf = requestAnimationFrame(renderLoop);
};

const isRunning = () => {
    return raf !== null;
};

const stop = () => {
    cancelAnimationFrame(raf);
    raf = null;
    btn.textContent = "Start";
};

const start = () => {
    renderLoop();
    btn.textContent = "Stop";
}

const btn = document.getElementById('play-pause');
btn.onclick = () => {
    if (isRunning()) {
        stop();
    } else {
        start();
    }
};

document.addEventListener("keydown", e => {
    if (keys[e.keyCode] >= 0) {
        chip8.key_down(keys[e.keyCode]);
    }
});

document.addEventListener("keyup", e => {
    if (keys[e.keyCode] >= 0) {
        chip8.key_up(keys[e.keyCode]);
    }
});

loadROM('BRIX');
