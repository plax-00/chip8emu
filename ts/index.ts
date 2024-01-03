import { Chip8 } from "chip8emu";
import { memory } from "chip8emu/chip8emu_bg.wasm"

const WIDTH = 64;
const HEIGHT = 32;
const PIXEL_SIZE = 15;
const CPU_DELAY = 2;

const chip8 = new Chip8();
const canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
canvas.width = WIDTH * PIXEL_SIZE;
canvas.height = HEIGHT * PIXEL_SIZE;
const ctx = canvas.getContext('2d');
const display = new Uint8Array(memory.buffer, chip8.get_display(), WIDTH * HEIGHT);

const romInput = document.getElementById('rom') as HTMLInputElement;
const loadButton = document.getElementById('load-button') as HTMLButtonElement;
let rom: Uint8Array;
let timeoutID: NodeJS.Timeout;

loadButton.addEventListener('click', async () => {
	rom = new Uint8Array(await romInput.files[0].arrayBuffer())
	if (timeoutID) clearTimeout(timeoutID);
	chip8.clear_memory();
	chip8.load_rom(rom);
	timeoutID = setTimeout(renderLoop, CPU_DELAY);
})


function getIndex(x: number, y: number) {
	return x + y * WIDTH;
}

function render() {
	ctx.beginPath();
	for (let y = 0; y < HEIGHT; y++) {
		for (let x = 0; x < WIDTH; x++) {
			ctx.fillStyle = display[getIndex(x, y)] ? "white" : "black";
			ctx.fillRect(x * PIXEL_SIZE, y * PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE)
		}
	}
	ctx.stroke();
}
render();

function renderLoop() {
	chip8.tick();
	render();

	timeoutID = setTimeout(renderLoop, CPU_DELAY);
}

const keysLeft = [
	'1', '2', '3', '4',
	'q', 'w', 'e', 'r',
	'a', 's', 'd', 'd',
	'z', 'x', 'c', 'v',
];
const keysRight = [
	'7', '8', '9', '0',
	'u', 'i', 'o', 'p',
	'j', 'k', 'l', ';',
	'm', ',', '.', '/',
];
const keyMap: any = {}

for (let i = 0; i < keysRight.length; i++) {
	keyMap[keysLeft[i]] = i
	keyMap[keysRight[i]] = i
}

function keyHandler (event: KeyboardEvent) {
	if (event.repeat) return;
	if (!keyMap || Object.keys(keyMap).length == 0) return;
	let key = event.key;
	if (!keysLeft.includes(key) && !keysRight.includes(key)) return;

	let state = event.type == 'keydown';
	chip8.update_key(keyMap[key], state);
}

onkeydown = keyHandler;
onkeyup = keyHandler;

