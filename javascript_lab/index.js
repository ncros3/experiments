window.onload = function () {
    let canvas = document.getElementById("target");
    let ctx = canvas.getContext("2d");
    let img = document.getElementById("front");
    ctx.drawImage(img, 0, 0);
}

setInterval(drawCircle(), 3000);

function drawCircle() {
    let canvas = document.getElementById("target");
    let ctx = canvas.getContext("2d");

    let rayon = 100;
    let circle_evolve = true;

    if (circle_evolve === true) {

        if (rayon >= 200) {
            rayon = 200;
            circle_evolve = false;
        } else {
            rayon += 10;
        }

    } else {

        if (rayon <= 100) {
            rayon = 100;
            circle_evolve = true;
        } else {
            rayon -= 10;
        }

    }

    ctx.beginPath();
    ctx.arc(1000, 600, rayon, 0, 2 * Math.PI);
    ctx.stroke();
}