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
    '15PUZZLE',
    'BLINKY',
    'BLITZ',
    'BRIX',
    'CONNECT4',
    'GUESS',
    'HIDDEN',
    'INVADERS',
    'MAZE',
    'MERLIN',
    'MISSILE',
    'PONG',
    'PONG2',
    'PUZZLE',
    'SYZYGY',
    'TANK',
    'TETRIS',
    'TICTAC',
    'UFO',
    'VBRIX',
    'VERS',
    'WIPEOFF'
];

const menu = document.getElementById('menu');
const romName = document.getElementById('rom');
const btn = document.getElementById('play-pause');
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

roms.forEach(rom => {
    const option = document.createElement('div');
    
    option.textContent = rom;
    option.classList.add('option');
    option.addEventListener('click', e => {
        menu.classList.remove('active');
        romName.textContent = e.target.textContent;        
        loadROM(e.target.textContent);
    });
    
    menu.appendChild(option);
});

document.getElementById('trigger').addEventListener('click', () => {
    menu.classList.toggle('active');
});

// paint canvas black
ctx.fillRect(0, 0, canvas.width, canvas.height);

const loadROM = async rom => {
    await fetch(`roms/${rom}`)
        .then(res => {
            if (res.ok) {
                return res.arrayBuffer();
            }
            throw new Error(`Error fetching ROM: (${res.status}) ${res.statusText}`);
        })
        .then(buf => {
            stop();
            chip8.reset();
            const mem = new Uint8Array(memory.buffer, chip8.get_memory(), 4096);
            const romData = new Uint8Array(buf);
            for (let i = 0; i < romData.length; i++) {
                mem[0x200 + i] = romData[i];
            }
        })
        .catch(console.error);
    ctx.fillRect(0, 0, canvas.width, canvas.height);
};

const drawGfx = () => {
    const gfx = new Uint8Array(memory.buffer, chip8.get_display(), 2048);
    const imgData = ctx.createImageData(canvas.width, canvas.height);
    for (let i = 0; i < gfx.length; i++) {
        imgData.data[i * 4] = gfx[i] ? 255 : 0;
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
    btn.textContent = 'Start';
};

const start = () => {
    renderLoop();
    btn.textContent = 'Stop';
};

const toggleEmulation = () => {
    if (isRunning()) {
        stop();
    } else {
        start();
    }
};

btn.addEventListener('click', toggleEmulation);

document.addEventListener('keydown', e => {
    if (keys[e.keyCode] >= 0) {
        chip8.key_down(keys[e.keyCode]);
    } else if (e.code === 'Enter') {
        toggleEmulation();
    }
});

document.addEventListener('keyup', e => {
    if (keys[e.keyCode] >= 0) {
        chip8.key_up(keys[e.keyCode]);
    }
});

loadROM('BRIX');
