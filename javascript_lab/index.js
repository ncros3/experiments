import * as kampos from "https://cdn.skypack.dev/kampos@0.5.1";

document.addEventListener("DOMContentLoaded", function() {

function loadImage (src) {
   return new Promise(resolve => {
       const img = new Image();
       img.crossOrigin = 'anonymous';

       img.onload = function () {
           resolve(this);
       };

       img.src = src;
   });
}
// get the image URLs
const imageFromSrc = document.querySelector('#source-from').src;
const imageToSrc = document.querySelector('#source-to').dataset.src;
// load images
const promisedImages = [
    loadImage(imageFromSrc),
    loadImage(imageToSrc)
];

const turbulence = kampos.effects.turbulence({ noise: kampos.noise.perlinNoise });

const WIDTH = 854;
const HEIGHT = 480;
const CELL_FACTOR = 4;
const AMPLITUDE = CELL_FACTOR / WIDTH;

turbulence.frequency = { x: AMPLITUDE, y: AMPLITUDE };
turbulence.octaves = 8;
turbulence.isFractal = true;

const mapTarget = document.createElement('canvas');
mapTarget.width = WIDTH;
mapTarget.height = HEIGHT;

const dissolveMap = new kampos.Kampos({ target: mapTarget, effects: [turbulence], noSource: true });

dissolveMap.draw();

const dissolve = kampos.transitions.dissolve();

dissolve.map = mapTarget;
dissolve.high = 0.3; // for liquid-like effect

const target = document.querySelector('#target');
const hippo = new kampos.Kampos({ target, effects: [dissolve] });

Promise.all(promisedImages).then(([fromImage, toImage]) => {
    hippo.setSource({ media: fromImage, width: WIDTH, height: HEIGHT });

    dissolve.to = toImage;
    dissolve.textures[1].update = true;
}).then(function () {
    hippo.play(time => {
        turbulence.time = time * 2;
        dissolveMap.draw();
        dissolve.progress = Math.abs(Math.sin(time * 2e-4)); // multiply time by a factor to slow it down a bit
    });
});

})